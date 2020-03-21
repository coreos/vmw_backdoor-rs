/// Low-bandwidth backdoor protocol.
use crate::{asm, backdoor};
use crate::{BackdoorGuard, VmwError};

impl BackdoorGuard {
    pub(crate) fn get_version(&mut self) -> Result<u32, VmwError> {
        let arg = asm::LowBandwidthBuf {
            eax: backdoor::BACKDOOR_MAGIC,
            ebx: 0,
            ecx: backdoor::COMMAND_GET_VERSION,
            edx: backdoor::BACKDOOR_PORT_LB,
            ebp: 0,
            edi: 0,
            esi: 0,
        };
        let mut res = asm::LowBandwidthBuf::default();
        asm::low_bw_in(&arg, &mut res);
        if res.ebx != backdoor::BACKDOOR_MAGIC {
            return Err("get_version failed".into());
        }
        Ok(res.ecx)
    }
}
