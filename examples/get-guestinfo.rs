use std::env;
use vmw_backdoor as vmw;

fn main() {
    let key = match env::args().collect::<Vec<_>>().get(1) {
        Some(val) => val.clone(),
        None => panic!("missing argument: key name"),
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

    match erpc.get_guestinfo(key.as_bytes()).unwrap() {
        Some(val) => {
            eprintln!("Got value for key '{}'.", key);
            println!("{}", String::from_utf8_lossy(&val));
        }
        None => panic!("Guestinfo property not found."),
    };
}
