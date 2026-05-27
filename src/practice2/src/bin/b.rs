use ac_library::FenwickTree;
use proconio::input;
use proconio::marker::Usize1;
fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      n: usize,
      q: usize,
      a: [usize; n],
    }

    let mut bit: FenwickTree<usize> = FenwickTree::new(n, 0);
    for i in 0..n {
        bit.add(i, a[i]);
    }
    for _ in 0..q {
        input! {
            num: usize,
        }
        if num == 0 {
            input! {
                p: usize,
                x: usize,
            }
            bit.add(p, x);
        } else {
            input! {
                l: usize,
                r: usize,
            }
            println!("{}", bit.sum(l..r));
        }
    }
}
