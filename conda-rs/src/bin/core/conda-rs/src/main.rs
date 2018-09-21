extern crate libconda_rs;

pub fn main() {
  let verstr =env!("CARGO_PKG_VERSION");
  println!("Version {}", verstr);
}
