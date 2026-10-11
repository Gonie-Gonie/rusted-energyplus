//! Reserve the Windows main-thread stack without changing floating-point controls.
fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").ok();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").ok();
    let link_arg = match (target_os.as_deref(), target_env.as_deref()) {
        (Some("windows"), Some("gnu")) => "-Wl,--stack,8388608",
        (Some("windows"), Some("msvc")) => "/STACK:8388608",
        _ => return,
    };
    println!("cargo::rustc-link-arg-bin=eplus-rs={link_arg}");
}
