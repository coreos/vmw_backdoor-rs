#![allow(dead_code)]

fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")
        .expect("missing $CARGO_CFG_TARGET_ARCH env variable");
    let target_os =
        std::env::var("CARGO_CFG_TARGET_OS").expect("missing $CARGO_CFG_TARGET_OS env variable");
    if target_arch == "x86_64" && target_os == "linux" {
        asm_x86_64_linux();
    };
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
