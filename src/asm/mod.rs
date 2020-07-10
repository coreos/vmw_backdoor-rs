//! FFI bridge to ASM functions.

use log::trace;

extern "C" {
    /// External, low-bandwidth IN.
    fn _vmw_backdoor_lb_in(arg: &LowBandwidthBuf, res: &mut LowBandwidthBuf);
    /// External, low-bandwidth OUT.
    fn _vmw_backdoor_lb_out(arg: &LowBandwidthBuf, res: &mut LowBandwidthBuf);

    /// External, high-bandwidth IN.
    fn _vmw_backdoor_hb_in(arg: &HighBandwidthBuf, res: &mut HighBandwidthBuf);
    /// External, high-bandwidth OUT.
    fn _vmw_backdoor_hb_out(arg: &HighBandwidthBuf, res: &mut HighBandwidthBuf);
}

/// Exchange buffer for low-bandwidth backdoor calls.
#[derive(Default)]
#[repr(C, packed)]
pub(crate) struct LowBandwidthBuf {
    pub(crate) eax: u32,
    pub(crate) ebx: u32,
    pub(crate) ecx: u32,
    pub(crate) edx: u32,
    pub(crate) ebp: u32,
    pub(crate) edi: u32,
    pub(crate) esi: u32,
}

/// Low-bandwidth IN.
pub(crate) fn low_bw_in(arg: &LowBandwidthBuf, res: &mut LowBandwidthBuf) {
    trace!(
        "lb_in, req: A={:x} - B={:x} - C={:x} - D={:x}",
        { arg.eax },
        { arg.ebx },
        { arg.ecx },
        { arg.edx }
    );

    unsafe { _vmw_backdoor_lb_in(arg, res) };

    trace!(
        "lb_in, resp: A={:x} - B={:x} - C={:x} - D={:x}",
        { res.eax },
        { res.ebx },
        { res.ecx },
        { res.edx }
    );
}

pub(crate) fn low_bw_out(arg: &LowBandwidthBuf, res: &mut LowBandwidthBuf) {
    trace!(
        "lb_out, req: {:x} - {:x} - {:x} - {:x}",
        { arg.eax },
        { arg.ebx },
        { arg.ecx },
        { arg.edx }
    );

    unsafe { _vmw_backdoor_lb_out(arg, res) };

    trace!(
        "lb_out, resp: {:x} - {:x} - {:x} - {:x}",
        { res.eax },
        { res.ebx },
        { res.ecx },
        { res.edx }
    );
}

/// Exchange buffer for high-bandwidth backdoor calls.
#[derive(Default)]
#[repr(C, packed)]
pub(crate) struct HighBandwidthBuf {
    pub(crate) eax: u32,
    pub(crate) ebx: u32,
    pub(crate) ecx: u32,
    pub(crate) edx: u32,
    pub(crate) ebp: u32,
    pub(crate) rdi: u64,
    pub(crate) rsi: u64,
}

pub(crate) fn hb_in(arg: &HighBandwidthBuf, res: &mut HighBandwidthBuf) {
    trace!(
        "hb_in, req: {:x} - {:x} - {:x} - {:x} - {:x}",
        { arg.eax },
        { arg.ebx },
        { arg.ecx },
        { arg.edx },
        { arg.rsi }
    );

    unsafe { _vmw_backdoor_hb_in(arg, res) };

    trace!(
        "hb_in, resp: {:x} - {:x} - {:x} - {:x} - {:x}",
        { res.eax },
        { res.ebx },
        { res.ecx },
        { res.edx },
        { res.rsi }
    );
}

pub(crate) fn hb_out(arg: &HighBandwidthBuf, res: &mut HighBandwidthBuf) {
    trace!(
        "hb_out, req: {:x} - {:x} - {:x} - {:x} - {:x}",
        { arg.eax },
        { arg.ebx },
        { arg.ecx },
        { arg.edx },
        { arg.rsi }
    );

    unsafe { _vmw_backdoor_hb_out(arg, res) };

    trace!(
        "hb_out, resp: {:x} - {:x} - {:x} - {:x} - {:x}",
        { res.eax },
        { res.ebx },
        { res.ecx },
        { res.edx },
        { res.rsi }
    );
}

/// Check whether this is running on VMware virtual CPU.
///
/// [Detection]:
///  * CPUID leaf 0x1 (ECX) contains the virtualization bit.
///  * CPUID leaf 0x4000_0000 (EBX+ECX+EDX) contains the vendor label.
///
/// [Detection]: https://kb.vmware.com/s/article/1009458
pub fn is_vmware_cpu() -> bool {
    use core::arch::x86_64::__cpuid;

    let leaf_1 = unsafe { __cpuid(0x0000_0001) };
    if (leaf_1.ecx & 0x8000_0000) != 0 {
        let leaf_vmw = unsafe { __cpuid(0x4000_0000) };
        let mut buf = Vec::with_capacity(12);
        buf.extend_from_slice(&leaf_vmw.ebx.to_le_bytes());
        buf.extend_from_slice(&leaf_vmw.ecx.to_le_bytes());
        buf.extend_from_slice(&leaf_vmw.edx.to_le_bytes());
        if buf.as_slice() == b"VMwareVMware" {
            return true;
        }
    }

    false
}
