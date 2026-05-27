use proconio::input;

fn main() {
  #[cfg(debug_assertions)]
  unsafe { backtrace_on_stack_overflow::enable()};
  input! {
    n: usize,
    a: [usize; n],
  }
}
