use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // Debian/Ubuntu and Nixpkgs libgsl do not link libgslcblas themselves.
    // Our code calls no cblas_* symbols directly, so the --as-needed that
    // rustc passes would drop a plain `rustc-link-lib=gslcblas`.  Check the
    // target OS, not the host OS that this build script is compiled for.
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        println!("cargo:rustc-link-arg=-Wl,--push-state,--no-as-needed,-lgslcblas,--pop-state");
    }
}
