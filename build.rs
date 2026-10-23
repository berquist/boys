use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // Debian/Ubuntu libgsl does not link libgslcblas itself.  Check the
    // target OS, not the host OS that this build script is compiled for.
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        println!("cargo:rustc-link-lib=gslcblas");
    }
}
