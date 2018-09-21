extern crate libconda_rs;

use libconda_rs::test;

fn main() {
  let verstr =env!("CARGO_PKG_VERSION");
  println!("Version {}", verstr);
  
  test();
}
