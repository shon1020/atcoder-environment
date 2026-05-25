use proconio::input;

use crate::btree_multiset::MultiSet;

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

    let mut first_set: MultiSet<usize> = MultiSet::new();
    let mut second_set: MultiSet<usize> = MultiSet::new();
    second_set.insert(x);
    for (a, b) in query {
        let ab = vec![a, b];
        for i in 0..=1 {
            if let Some(&top) = second_set.first() {
                if ab[i] >= top {
                    second_set.insert(ab[i]);
                } else {
                    first_set.insert(ab[i]);
                }
            } else {
                first_set.insert(ab[i]);
            }

            if first_set.size() > second_set.size() {
                second_set.insert(first_set.pop_last().unwrap().clone());
            } else if first_set.size() + 1 < second_set.size() {
                first_set.insert(second_set.pop_first().unwrap().clone());
            }
        }
        let ans = second_set.first().unwrap().clone();
        println!("{}", ans);
    }
}

pub mod btree_multiset {
    use std::{
        borrow::Borrow,
        collections::{
            btree_map::{self},
            BTreeMap,
        },
        ops::{Bound, RangeBounds},
    };
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MultiSet<T> {
        size: usize,
        map: BTreeMap<T, usize>,
    }

    impl<T: Ord> Default for MultiSet<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: Ord> From<Vec<T>> for MultiSet<T> {
        fn from(vec: Vec<T>) -> Self {
            let size = vec.len();
            let mut btree_map = BTreeMap::new();

            for key in vec {
                *btree_map.entry(key).or_insert(0) += 1;
            }

            Self {
                size: size,
                map: btree_map,
            }
        }
    }
    impl<T: Ord> MultiSet<T> {
        pub fn new() -> Self {
            Self {
                size: 0,
                map: BTreeMap::new(),
            }
        }

        pub fn clear(&mut self) {
            self.size = 0;
            self.map.clear();
        }
        // 指定した値があるか確認
        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
        {
            self.map.contains_key(value)
        }

        // 挿入する
        pub fn insert(&mut self, key: T) {
            self.size += 1;
            *self.map.entry(key).or_insert(0) += 1;
        }

        //最小値の値を返す
        pub fn first(&self) -> Option<&T> {
            if let Some((key, _)) = self.map.iter().next() {
                Some(key)
            } else {
                None
            }
        }

        //最大値
        pub fn last(&self) -> Option<&T> {
            if let Some((key, _)) = self.map.iter().next_back() {
                Some(key)
            } else {
                None
            }
        }

        //空か確認
        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }

        //要素の種類の数
        pub fn len(&self) -> usize {
            self.map.len()
        }
        //要素の個数
        pub fn size(&self) -> usize {
            self.size
        }

        pub fn merge(&mut self, other: &mut MultiSet<T>)
        where
            T: Clone,
        {
            self.size += other.size;

            for (key, value) in other.map.iter() {
                if let Some(prev) = self.map.get_mut(key) {
                    *prev += value;
                } else {
                    self.map.insert(key.clone(), *value);
                }
            }
        }

        //最小値を取る
        pub fn pop_first(&mut self) -> Option<T>
        where
            T: Ord + Clone,
        {
            let mut entry = self.map.first_entry()?;
            self.size -= 1;
            if *entry.get() > 1 {
                *entry.get_mut() -= 1;
                Some(entry.key().clone())
            } else {
                let (key, _) = entry.remove_entry();
                Some(key)
            }
        }

        //最大値を取る
        pub fn pop_last(&mut self) -> Option<T>
        where
            T: Ord + Clone,
        {
            let mut entry = self.map.last_entry()?;
            self.size -= 1;
            if *entry.get() > 1 {
                *entry.get_mut() -= 1;
                Some(entry.key().clone())
            } else {
                let (key, _) = entry.remove_entry();
                Some(key)
            }
        }

        //特定の要素を一つ削除っ

        pub fn remove(&mut self, value: &T) -> bool
        where
            T: Clone,
        {
            self.map.entry(value.clone()).and_modify(|v| *v -= 1);
            if let Some(&cnt) = self.map.get(&value) {
                if cnt == 0 {
                    self.map.remove(&value);
                }

                self.size -= 1;
                true
            } else {
                false
            }
        }

        // x以上の最小値
        pub fn lower_bound<Q>(&self, bound: Bound<&Q>) -> Option<&T>
        where
            T: Borrow<Q>,
            Q: Ord,
        {
            match bound {
                Bound::Unbounded => unreachable!(),
                _ => {
                    if let Some((key, _)) = self.map.range((bound, Bound::Unbounded)).next() {
                        Some(key)
                    } else {
                        None
                    }
                }
            }
        }

        // x以下の最大値
        pub fn upper_bound<Q>(&self, bound: Bound<&Q>) -> Option<&T>
        where
            T: Borrow<Q>,
            Q: Ord,
        {
            match bound {
                Bound::Unbounded => unreachable!(),
                _ => {
                    if let Some((key, _)) = self.map.range((Bound::Unbounded, bound)).next_back() {
                        Some(key)
                    } else {
                        None
                    }
                }
            }
        }

        // iter関数を定義
        pub fn iter(&self) -> Iter<'_, T> {
            Iter {
                range: self.range(..),
            }
        }

        //range関数wp定義
        pub fn range<U: ?Sized, R>(&self, range: R) -> Range<'_, T>
        where
            U: Ord,
            T: Borrow<U> + Ord,
            R: RangeBounds<U>,
        {
            Range {
                last: None,
                counter: 0,
                range: self.map.range(range),
            }
        }

        //指定した要素の個数を返す
        pub fn count<Q>(&self, key: &Q) -> usize
        where
            T: Borrow<Q>,
            Q: Ord,
        {
            if let Some(&cnt) = self.map.get(key) {
                cnt
            } else {
                0
            }
        }
    }
    impl<T: Ord> FromIterator<T> for MultiSet<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> MultiSet<T> {
            let mut set = MultiSet::new();
            for v in iter {
                set.insert(v);
            }
            set
        }
    }

    #[derive(Debug, Clone, Default)]
    //通常のBTreeMapのRangeでは重複を全て操作できないのでRangeのラッパーは必要
    pub struct Range<'a, T>
    where
        T: 'a,
    {
        last: Option<&'a T>,
        counter: usize,
        range: btree_map::Range<'a, T, usize>,
    }

    impl<'a, T> Iterator for Range<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.counter == 0 {
                if let Some((key, cnt)) = self.range.next() {
                    self.last = Some(key);
                    self.counter = cnt - 1;
                    Some(key)
                } else {
                    None
                }
            } else {
                self.counter -= 1;
                self.last
            }
        }
    }

    impl<'a, T> DoubleEndedIterator for Range<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.counter == 0 {
                if let Some((key, &cnt)) = self.range.next_back() {
                    self.last = Some(key);
                    self.counter = cnt;
                    Some(key)
                } else {
                    None
                }
            } else {
                self.counter -= 1;
                self.last
            }
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct Iter<'a, T>
    where
        T: 'a,
    {
        range: Range<'a, T>,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            self.range.next()
        }
    }

    impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.range.next_back()
        }
    }
}
