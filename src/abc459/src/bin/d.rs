use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      t: usize,
    }

    for _ in 0..t {
        input! {
            s: Chars,
        }

        let mut cnt_vec = vec![0; 27];
        let n = s.len();
        for c in s {
            cnt_vec[c as usize - 'a' as usize + 1] += 1;
        }

        if *cnt_vec.iter().max().unwrap() > (n + 1) / 2 {
            println!("No");
            continue;
        }
        let mut prev: usize = 0;
        let mut ans = Vec::new();
        loop {
            let mut v_max = 0;
            let mut idx = 0;

            for i in 1..27 {
                if i == prev {
                    continue;
                }
                if cnt_vec[i as usize] > v_max {
                    idx = i;
                    v_max = cnt_vec[i as usize];
                }
            }

            if idx == 0 {
                break;
            }
            cnt_vec[idx as usize] -= 1;
            let st = (b'a' + (idx - 1) as u8) as char;
            ans.push(st);
            prev = idx;
        }
        println!("Yes");
        println!("{}", ans.iter().join(""));
    }
}

