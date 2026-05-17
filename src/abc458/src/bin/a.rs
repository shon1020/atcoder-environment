use proconio::{input, marker::Chars};
use itertools::itertools;
fn main() {
  #[cfg(debug_assertions)]
  unsafe { backtrace_on_stack_overflow::enable()};
  input! {
    s: Chars,
    n: Chars,
  }
  let mut x = 12;
  println!("{}", s[1..2*n+1].iter().join(""));
}
