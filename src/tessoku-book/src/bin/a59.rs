use ac_library::{fenwicktree, FenwickTree};
use proconio::{input, marker::Usize1};
fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      n: usize,
      q: usize,
    }

    let mut bit: FenwickTree<i64> = fenwicktree::FenwickTree::new(n, 0_i64);
    for _ in 0..q {
        input! {
            num: usize,
        }

        if num == 1 {
            input! {
                pos: Usize1,
                x:i64,
            }
            let cur = bit.sum(pos..pos + 1);
            bit.add(pos, x - cur);
        } else if num == 2 {
            input! {
                l: Usize1,
                r: Usize1,
            }

            println!("{}", bit.sum(l..r));
        }
    }
}
