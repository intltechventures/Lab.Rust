
use std::fmt;

// https://doc.rust-lang.org/std/fmt/

fn main() {
  let mut i = 0;
  while i < 256 {
    // let s = format!("%d /t%X /t%b", i, i, i);
    println!(i);
    i += 1;
  }
}
