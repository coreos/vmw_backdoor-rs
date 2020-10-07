//! VMware backdoor protocol
//!
//! Ref: https://github.com/vmware/open-vm-tools/blob/stable-11.0.5/open-vm-tools/lib/include/backdoor_def.h

use crate::{EnhancedChan, VmwError};

/// Magic value for all backdoor commands ('VMXh').
pub(crate) const BACKDOOR_MAGIC: u32 = 0x564D5868;

/// Low-bandwidth backdoor port.
pub(crate) const BACKDOOR_PORT_LB: u32 = 0x5658;
/// High-bandwidth backdoor port.
pub(crate) const BACKDOOR_PORT_HB: u32 = 0x5659;

pub(crate) const COMMAND_GET_VERSION: u32 = 0x0A;
pub(crate) const COMMAND_ERPC: u32 = 0x1E;

/// Try to acquire access to the backdoor, but do NOT probe its presence.
///
/// On Linux, this tries to change I/O access level via `iopl()`. That requires
/// running with `CAP_SYS_RAWIO` capability and it is not compatible with
/// `kernel_lockdown`.
pub fn access_backdoor_privileged() -> Result<BackdoorGuard, VmwError> {
    BackdoorGuard::change_io_access(true)?;
    Ok(BackdoorGuard {
        release_on_drop: true,
    })
}

/// Try to acquire access to the backdoor, but do NOT probe its presence.
///
/// Wherever possible, use `access_backdoor_privileged()` instead.
pub fn access_backdoor() -> Result<BackdoorGuard, VmwError> {
    Ok(BackdoorGuard {
        release_on_drop: false,
    })
}

/// Try to acquire access to the backdoor, and probe its presence.
///
/// On Linux, this tries to change I/O access level via `iopl()`. That requires
/// running with `CAP_SYS_RAWIO` capability and it is not compatible with
/// `kernel_lockdown`.
pub fn probe_backdoor_privileged() -> Result<BackdoorGuard, VmwError> {
    BackdoorGuard::change_io_access(true)?;
    let mut guard = BackdoorGuard {
        release_on_drop: true,
    };
    guard.probe_vmware_backdoor()?;
    Ok(guard)
}

/// Try to acquire access to the backdoor, and probe its presence.
///
/// Wherever possible, use `probe_backdoor_privileged()` instead.
pub fn probe_backdoor() -> Result<BackdoorGuard, VmwError> {
    let mut guard = BackdoorGuard {
        release_on_drop: false,
    };
    guard.probe_vmware_backdoor()?;
    Ok(guard)
}

/// Guard for an open backdoor.
///
/// This can be acquired via [`access_backdoor`](fn.access_backdoor.html) or
/// [`probe_backdoor`](fn.probe_backdoor.html).
#[derive(Debug)]
pub struct BackdoorGuard {
    release_on_drop: bool,
}

impl BackdoorGuard {
    /// Check whether the VMware backdoor is accessible.
    pub fn probe_vmware_backdoor(&mut self) -> Result<(), VmwError> {
        self.get_version().and(Ok(()))
    }

    /// Try to release backdoor access.
    pub fn release_access(self) -> Result<(), Self> {
        let mut guard = self;
        if Self::change_io_access(false).is_err() {
            return Err(guard);
        }

        guard.release_on_drop = false;
        drop(guard);
        Ok(())
    }

    /// Open channel for enhanced-RPC.
    pub fn open_enhanced_chan(&mut self) -> Result<EnhancedChan, VmwError> {
        EnhancedChan::open(self)
    }

    /// Try to change I/O ports access level.
    pub(crate) fn change_io_access(acquire: bool) -> Result<(), VmwError> {
        // NOTE(lucab): `ioperm()` is not enough here, as the backdoor
        //  protocol uses a dynamic range of I/O ports.
        let level = if acquire { 0b11 } else { 0b00 };
        let err = unsafe { libc::iopl(level) };
        if err != 0 {
            let err_code = errno::errno();
            return Err(format!("iopl failed: {} (errno: {})", err_code, err_code.0).into());
        };

        Ok(())
    }
}

impl Drop for BackdoorGuard {
    fn drop(&mut self) {
        if self.release_on_drop {
            if let Err(e) = Self::change_io_access(false) {
                log::error!("failed to release backdoor access: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_string() {
        assert_eq!(BACKDOOR_MAGIC, u32::from_be_bytes(*b"VMXh"));
    }
}
