use cargo_snippet::snippet;

#[snippet(name = "btree_multiset")]
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
        // 指定した値があるか確認
        pub fn contains<Q>(&self, value: &T) -> bool
        where
            T: Borrow<T>,
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

        //長さ
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
            T: Clone,
        {
            if self.is_empty() {
                None
            } else {
                self.size -= 1;
                let first = self.first().unwrap().clone();
                self.remove(&first);
                Some(first)
            }
        }

        //最大値を取る
        pub fn pop_last(&mut self) -> Option<T>
        where
            T: Clone,
        {
            if self.is_empty() {
                None
            } else {
                self.size -= 1;
                let last = self.last().unwrap().clone();
                self.remove(&last);
                Some(last)
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
