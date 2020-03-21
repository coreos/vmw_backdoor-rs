use std::env;
use vmw_backdoor as vmw;

fn main() {
    let msg = match env::args().collect::<Vec<_>>().get(1) {
        Some(val) => val.clone(),
        None => "Hello world (from vmw_backdoor)".to_string(),
    };

    let is_vmw = vmw::is_vmware_cpu();
    eprintln!("VMware CPU detected: {}.", is_vmw);
    if !is_vmw {
        panic!("Hypervisor not present");
    }

    let mut backdoor = vmw::probe_backdoor().unwrap();
    eprintln!("Got backdoor access.");

    let mut erpc = backdoor.open_enhanced_chan().unwrap();
    eprintln!("Got ERPC channel: {:?}.", erpc);

    erpc.log(&msg).unwrap();
    eprintln!("Sent log message: {}.", msg);
}
