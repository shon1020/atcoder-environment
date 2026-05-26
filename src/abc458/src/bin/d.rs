use ac_library::{fenwicktree, FenwickTree};
use num_traits::pow;
use proconio::input;
use superslice::Ext;

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      x: usize,
      q: usize,
      query: [(usize, usize); q],
    }
    //座標圧縮
    let mut vec = vec![x];
    let pre_q = query.clone();
    for &(a, b) in &pre_q {
        vec.push(a);
        vec.push(b);
    }
    vec.sort();
    vec.dedup();
    let n = vec.len();

    let mut bit: FenwickTree<usize> = FenwickTree::new(n, 0);
    let x_com = to_idx(&vec, &x);
    bit.add(x_com, 1);
    let mut total = 1; //要素の総数

    for (a, b) in query {
        let a = to_idx(&vec, &a);
        let b = to_idx(&vec, &b);

        bit.add(a, 1);
        total += 1;
        bit.add(b, 1);
        total += 1;
        let mut left: usize = 0;
        let mut right: usize = n;

        while left < right {
            let mid = (left + right) / 2;
            if bit.sum(0..=mid) >= ((total + 1) / 2) {
                right = mid
            } else {
                left = mid + 1;
            }
        }
        println!("{}", vec[left]);
    }
}

fn to_idx(vec: &Vec<usize>, x: &usize) -> usize {
    vec.lower_bound(x)
}
