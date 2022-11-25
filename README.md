# vmw\_backdoor

[![crates.io](https://img.shields.io/crates/v/vmw_backdoor.svg)](https://crates.io/crates/vmw_backdoor)
[![Documentation](https://docs.rs/vmw_backdoor/badge.svg)](https://docs.rs/vmw_backdoor)

A pure-Rust library for VMware host-guest protocol ("VMXh backdoor").

This library provides helpers to access and use the VMware backdoor from a
guest VM. It allows bi-directional interactions with the VMWare hypervisor
and host environment.

The "backdoor" protocol does not have official specifications, but it has been
widely analyzed and there are multiple projects documenting it:
  1. [https://github.com/vmware/open-vm-tools/blob/stable-11.0.5/open-vm-tools/lib/include/backdoor_def.h][1]
  2. [https://wiki.osdev.org/VMware_tools][2]
  3. [https://sysprogs.com/legacy/articles/kdvmware/guestrpc.shtml][3]
  4. [https://github.com/vmware/vmw-guestinfo/tree/master/bdoor][4]
  5. [https://sites.google.com/site/chitchatvmback/backdoor][5]

[1]: https://github.com/vmware/open-vm-tools/blob/stable-11.0.5/open-vm-tools/lib/include/backdoor_def.h
[2]: https://wiki.osdev.org/VMware_tools
[3]: https://sysprogs.com/legacy/articles/kdvmware/guestrpc.shtml
[4]: https://github.com/vmware/vmw-guestinfo/tree/master/bdoor
[5]: https://sites.google.com/site/chitchatvmback/backdoor

## Example

```rust
let is_vmw = vmw_backdoor::is_vmware_cpu();
println!("VMware CPU detected: {}.", is_vmw);

let mut guard = vmw_backdoor::access_backdoor().unwrap();
println!("Raised I/O access to reach backdoor port.");

let found = guard.probe_vmware_backdoor().unwrap_or(false);
println!("VMware backdoor detected: {}.", found);

let mut erpc = guard.open_enhanced_chan().unwrap();
let key = "guestinfo.ignition.config.data";
let guestinfo = erpc.get_guestinfo(key.as_bytes()).unwrap();

if let Some(val) = guestinfo {
    println!("Got value for key '{}':", key);
    println!("{}", String::from_utf8_lossy(&val));
};
```

Some more examples are available under [examples](examples).

## License

Licensed under either of

 * MIT license - <http://opensource.org/licenses/MIT>
 * Apache License, Version 2.0 - <http://www.apache.org/licenses/LICENSE-2.0>

at your option.
