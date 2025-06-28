use std::env;

fn main() {
  if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("./assets/convert-screenshot-cli.ico");
    res.compile().unwrap();
  }
}
