use ac_library::FenwickTree;
use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};
define_queries! {
    enum Query: usize {
        1 => Output { k: usize },
        2 => Reverse { k: usize },
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }

    let mut vec: Vec<i64> = (1..=2 * n).into_iter().map(|i| i as i64).collect_vec();
    let mut bit: FenwickTree<i64> = FenwickTree::new(2 * n, 0i64);

    for query in queries {
        match query {
            Query::Output { k } => {
                let ans = if bit.sum(0..k) % 2 == 0 {
                    vec[k - 1]
                } else {
                    vec[2 * n - k]
                };
                println!("{}", ans);
            }
            Query::Reverse { k } => {
                bit.add(n - k, 1);
                bit.add(n + k, -1);
            }
        }
    }
}

#[macro_export]
macro_rules! define_queries {
  ($( $(#[$attr:meta])* enum $enum_name:ident : $sig:ty { $( $pattern:pat => $variant:ident $( { $($name:ident : $marker:ty $(,)?),* } )? $(,)?),* } )*) => {
    $(
      $(#[$attr])*
      enum $enum_name {
        $(
          $variant $( {
            $( $name : <$marker as proconio::source::Readable>::Output ),*
          } )?
        ),*
      }

      impl proconio::source::Readable for $enum_name {
        type Output = Self;
        fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self {
          #![allow(unreachable_patterns)]
          match <$sig as proconio::source::Readable>::read(source) {
            $(
              $pattern => $enum_name::$variant $( {
                $( $name: <$marker as proconio::source::Readable>::read(source) ),*
              } )?
            ),*
            , _ => unreachable!()
          }
        }
      }
    )*
  }
}
