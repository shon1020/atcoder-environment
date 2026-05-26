use proconio::input;
use superslice::Ext;

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      n: usize,
      a: [usize; n],
    }

    let mut b = a.clone();

    b.sort();
    b.dedup();
    for i in 0..n {
        let ans = b.lower_bound(&a[i]);
        println!("{}", ans);
    }
}
