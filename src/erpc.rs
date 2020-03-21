//! Enhanced-RPC protocol (ERPC).
//!
//! Ref: https://sites.google.com/site/chitchatvmback/backdoor

use crate::{asm, backdoor};
use crate::{BackdoorGuard, VmwError};

/// Magic value for all enhanced-RPC commands ('RPCI').
const RPCI_MAGIC: u32 = 0xC9435052;

/// Magic value for all enhanced-RPC data transfers.
const ERPC_DATA_MAGIC: u32 = 0x0001_0000;

/// ERPC subcommand: open.
const ERPC_OPEN: u32 = 0x0;
/// ERPC subcommand: send command length.
const ERPC_SEND_CMD_LEN: u32 = 0x1;
/// ERPC subcommand: receive data length.
const ERPC_RECV_REPLY_LEN: u32 = 0x3;
/// ERPC subcommand: clone.
const ERPC_CLOSE: u32 = 0x6;

/// Channel for enhanced-RPC transfers.
///
/// This can be acquired via [`BackdoorGuard::open_enhanced_chan`](struct.BackdoorGuard.html#method.open_enhanced_chan).
#[derive(Debug)]
pub struct EnhancedChan<'a> {
    guard: &'a mut BackdoorGuard,
    pub(crate) chan_id: u32,
    pub(crate) cookie1: u32,
    pub(crate) cookie2: u32,
}

impl<'a> Drop for EnhancedChan<'a> {
    fn drop(&mut self) {
        if self.close_channel().is_err() {
            log::warn!("failed to close enhanced-RPC channel");
        };
    }
}

impl<'a> EnhancedChan<'a> {
    /// Open an enhanced-RPC channel.
    pub fn open(guard: &'a mut BackdoorGuard) -> Result<Self, VmwError> {
        let arg = asm::LowBandwidthBuf {
            eax: backdoor::BACKDOOR_MAGIC,
            ebx: RPCI_MAGIC,
            ecx: erpc_subcommand(ERPC_OPEN),
            edx: backdoor::BACKDOOR_PORT_LB,
            ..Default::default()
        };
        let mut res = asm::LowBandwidthBuf::default();
        asm::low_bw_out(&arg, &mut res);
        if res.ecx == 0 {
            return Err("erpc open failed".into());
        }

        let ch = EnhancedChan {
            guard,
            chan_id: res.edx,
            cookie1: res.esi,
            cookie2: res.edi,
        };

        Ok(ch)
    }

    /// Close the channel.
    ///
    /// On error, this returns back the channel.
    pub fn close(self) -> Result<(), Self> {
        let mut erpc = self;
        match erpc.close_channel() {
            Ok(_) => Ok(()),
            Err(_) => Err(erpc),
        }
    }

    fn close_channel(&mut self) -> Result<(), VmwError> {
        let arg = asm::LowBandwidthBuf {
            eax: backdoor::BACKDOOR_MAGIC,
            ebx: 0,
            ecx: erpc_subcommand(ERPC_CLOSE),
            edx: erpc_cmd_channel(self.chan_id),
            esi: self.cookie1,
            edi: self.cookie2,
            ebp: 0,
        };
        let mut res = asm::LowBandwidthBuf::default();
        asm::low_bw_out(&arg, &mut res);
        if res.ecx == 0 {
            return Err("erpc close_channel failed".into());
        }

        Ok(())
    }

    fn send_command_len(&mut self, cmd_len: u32) -> Result<(), VmwError> {
        let arg = asm::LowBandwidthBuf {
            eax: backdoor::BACKDOOR_MAGIC,
            ebx: cmd_len,
            ecx: erpc_subcommand(ERPC_SEND_CMD_LEN),
            edx: erpc_cmd_channel(self.chan_id),
            esi: self.cookie1,
            edi: self.cookie2,
            ebp: 0,
        };
        let mut res = asm::LowBandwidthBuf::default();
        asm::low_bw_out(&arg, &mut res);
        if res.ecx == 0 {
            return Err("erpc send_command_len failed".into());
        }

        Ok(())
    }

    fn send_command_data(&mut self, command: &[u8], cmd_len: u32) -> Result<(), VmwError> {
        let arg = asm::HighBandwidthBuf {
            eax: backdoor::BACKDOOR_MAGIC,
            ebx: ERPC_DATA_MAGIC,
            ecx: cmd_len,
            edx: erpc_data_channel(self.chan_id),
            ebp: self.cookie1,
            rdi: self.cookie2 as u64,
            rsi: command.as_ptr() as u64,
        };
        let mut res = asm::HighBandwidthBuf::default();
        asm::hb_out(&arg, &mut res);
        if res.ebx == 0 {
            return Err("erpc send_command_len failed".into());
        }

        Ok(())
    }

    fn recv_reply_len(&mut self) -> Result<usize, VmwError> {
        let arg = asm::LowBandwidthBuf {
            eax: backdoor::BACKDOOR_MAGIC,
            ebx: 0,
            ecx: erpc_subcommand(ERPC_RECV_REPLY_LEN),
            edx: erpc_cmd_channel(self.chan_id),
            esi: self.cookie1,
            edi: self.cookie2,
            ebp: 0,
        };
        let mut res = asm::LowBandwidthBuf::default();
        asm::low_bw_out(&arg, &mut res);
        if res.ecx == 0 {
            return Err("erpc recv_reply_len failed".into());
        }

        Ok(res.ebx as usize)
    }

    fn recv_reply_data(&mut self, reply_len: usize) -> Result<Vec<u8>, VmwError> {
        let buf = vec![0; reply_len];
        let arg = asm::HighBandwidthBuf {
            eax: backdoor::BACKDOOR_MAGIC,
            ebx: ERPC_DATA_MAGIC,
            ecx: reply_len as u32,
            edx: erpc_data_channel(self.chan_id),
            ebp: self.cookie2,
            rdi: buf.as_slice().as_ptr() as u64,
            rsi: self.cookie1 as u64,
        };
        let mut res = asm::HighBandwidthBuf::default();
        asm::hb_in(&arg, &mut res);
        if res.ebx == 0 {
            return Err("erpc recv_reply_data failed".into());
        }

        Ok(buf)
    }

    /// Retrieve a guestinfo property.
    pub fn get_guestinfo(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, VmwError> {
        let mut command = b"info-get ".to_vec();
        command.extend(key);
        command.push(b'\0');
        let cmd_len = command.len() as u32;
        self.send_command_len(cmd_len)?;
        self.send_command_data(&command, cmd_len)?;
        let len = self.recv_reply_len()?;
        let mut buf = self.recv_reply_data(len)?;

        if buf.len() < 2 {
            return Err(format!("reply too short ({} bytes)", buf.len()).into());
        }

        if buf.remove(0) == b'0' {
            Ok(None)
        } else {
            // Strip whitespace separator.
            buf.remove(0);
            Ok(Some(buf))
        }
    }

    /// Log a message.
    pub fn log(&mut self, msg: &str) -> Result<(), VmwError> {
        let mut command = b"log ".to_vec();
        command.extend(msg.as_bytes());
        command.push(b'\0');
        let cmd_len = command.len() as u32;
        self.send_command_len(cmd_len)?;
        self.send_command_data(&command, cmd_len)?;
        let len = self.recv_reply_len()?;
        let buf = self.recv_reply_data(len)?;

        if buf.is_empty() {
            return Err("empty reply".into());
        }

        match buf[0] {
            b'1' => Ok(()),
            _ => Err("erpc log failed".into()),
        }
    }

    /// Report agent (unmanaged) type.
    pub fn report_agent(&mut self) -> Result<(), VmwError> {
        const TOOLS_VERSION_UNMANAGED: i32 = std::i32::MAX;
        let mut command = format!("tools.set.version {}", TOOLS_VERSION_UNMANAGED)
            .as_bytes()
            .to_vec();
        command.push(b'\0');
        let cmd_len = command.len() as u32;
        self.send_command_len(cmd_len)?;
        self.send_command_data(&command, cmd_len)?;
        let len = self.recv_reply_len()?;
        let buf = self.recv_reply_data(len)?;

        if buf.is_empty() {
            return Err("empty reply".into());
        }

        match buf[0] {
            b'1' => Ok(()),
            _ => Err("erpc tools.set.version failed".into()),
        }
    }
}

/// Format an ERPC subcommand.
fn erpc_subcommand(subcmd: u32) -> u32 {
    (subcmd << 16) | backdoor::COMMAND_ERPC
}

/// Format an ERPC channel for control commands.
fn erpc_cmd_channel(chan_id: u32) -> u32 {
    (chan_id & 0xffff_0000) | backdoor::BACKDOOR_PORT_LB
}

/// Format an ERPC channel for data tranfer.
fn erpc_data_channel(chan_id: u32) -> u32 {
    (chan_id & 0xffff_0000) | backdoor::BACKDOOR_PORT_HB
}
