use std::env;
use vmw_backdoor as vmw;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
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

    let mut backdoor = vmw::probe_backdoor_privileged().unwrap();
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

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
fn main() {
    eprintln!("Unsupported target");
}
