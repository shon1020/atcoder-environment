use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};
use std::ops::{BitXorAssign, Bound, RangeBounds};
define_queries! {
    enum Query: usize {
        1 => Change {x: usize, y: usize },
        2 => Calc {x: usize, y: usize}
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        xs: [usize; n],
        queries: [Query; q],
    }

    let mut bit = FenwickTree::new(n + 1, 0usize);

    for (i, &a) in xs.iter().enumerate() {
        bit.add(i + 1, a);
    }

    for query in queries {
        match query {
            Query::Change { x, y } => {
                bit.add(x, y);
            }
            Query::Calc { x, y } => {
                println!("{}", bit.sum(0..=y) ^ bit.sum(0..=x - 1));
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

#[derive(Clone)]
pub struct FenwickTree<T> {
    n: usize,
    ary: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign<T> + BitXorAssign> FenwickTree<T> {
    pub fn new(n: usize, e: T) -> Self {
        FenwickTree {
            n,
            ary: vec![e.clone(); n],
            e,
        }
    }
    pub fn accum(&self, mut idx: usize) -> T {
        let mut sum = self.e.clone();
        while idx > 0 {
            sum ^= self.ary[idx - 1].clone();
            idx &= idx - 1;
        }
        sum
    }
    /// performs data[idx] += val;
    pub fn add<U: Clone + BitXorAssign>(&mut self, mut idx: usize, val: U)
    where
        T: std::ops::AddAssign<U> + BitXorAssign<U>,
    {
        let n = self.n;
        idx += 1;
        while idx <= n {
            self.ary[idx - 1] ^= val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }
    /// Returns data[l] + ... + data[r - 1].
    pub fn sum<R>(&self, range: R) -> T
    where
        T: std::ops::Sub<Output = T>,
        R: RangeBounds<usize>,
    {
        let r = match range.end_bound() {
            Bound::Included(r) => r + 1,
            Bound::Excluded(r) => *r,
            Bound::Unbounded => self.n,
        };
        let l = match range.start_bound() {
            Bound::Included(l) => *l,
            Bound::Excluded(l) => l + 1,
            Bound::Unbounded => return self.accum(r),
        };
        self.accum(r) - self.accum(l)
    }
}
