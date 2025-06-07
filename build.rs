use std::env;

fn main() {
  let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
  if target_os != "windows" {
    println!("cargo:warning=This crate only supports Windows. Aborting build.");
    panic!("This crate only supports Windows target.")
  }
}
