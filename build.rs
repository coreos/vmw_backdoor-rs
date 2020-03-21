#![allow(dead_code)]

fn main() {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    asm_x86_64_linux();
}

fn asm_x86_64_linux() {
    // build directory for this crate
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    let src = "src/asm/x86_64-linux.s";

    // assemble source
    cc::Build::new().file(src).compile("asm");

    // rebuild if source changed
    println!("cargo:rerun-if-changed={}", src);
}
