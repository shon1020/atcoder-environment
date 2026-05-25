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

#[cfg(test)]
mod tests {
    use std::ops::Bound::{Excluded, Included};

    use crate::multiset::btree_multiset::MultiSet;

    #[test]
    fn test_new() {
        let set: MultiSet<i32> = MultiSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_default() {
        let set: MultiSet<i32> = MultiSet::default();

        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_from() {
        let vec = vec![1, 2, 3, 4];
        let set: MultiSet<i32> = MultiSet::from(vec);

        assert!(!set.is_empty());
        assert_eq!(set.len(), 4);
    }
    #[test]
    fn test_contains() {
        let set: MultiSet<i32> = vec![1, 1, 2, 3, 4].into_iter().collect();

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(!set.contains(&5));
    }

    #[test]
    fn test_insert() {
        let mut set: MultiSet<i32> = MultiSet::new();

        set.insert(1);
        assert!(!set.is_empty());
        assert_eq!(set.len(), 1);
        assert_eq!(set.size(), 1);
        assert!(set.contains(&1));
        assert_eq!(set.count(&1), 1);

        set.insert(1);
        assert!(!set.is_empty());
        assert_eq!(set.len(), 1);
        assert_eq!(set.size(), 2);
        assert!(set.contains(&1));
        assert_eq!(set.count(&1), 2);

        set.insert(2);
        assert!(!set.is_empty());
        assert_eq!(set.len(), 2);
        assert_eq!(set.size(), 3);
        assert!(set.contains(&2));
        assert_eq!(set.count(&2), 1);
    }

    #[test]
    fn test_first_last() {
        let mut set: MultiSet<i32> = MultiSet::new();

        assert_eq!(set.first(), None);
        assert_eq!(set.last(), None);

        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);

        assert_eq!(set.first(), Some(&1));
        assert_eq!(set.last(), Some(&4));
    }

    #[test]
    fn test_clear() {
        let mut set: MultiSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
        set.clear();
        assert_eq!(set, MultiSet::from(vec![]));
    }

    #[test]
    fn test_merge() {
        let mut set1: MultiSet<i32> = vec![1, 1, 2].into_iter().collect();
        let mut set2: MultiSet<i32> = vec![2, 2, 3].into_iter().collect();

        set1.merge(&mut set2);
        assert_eq!(set1, MultiSet::from(vec![1, 1, 2, 2, 2, 3]));
        assert_eq!(set1.len(), 3);
        assert_eq!(set1.size(), 6);
        assert!(set1.contains(&1));
        assert!(set1.contains(&2));
        assert!(set1.contains(&3));
        assert_eq!(set1.count(&1), 2);
        assert_eq!(set1.count(&2), 3);
        assert_eq!(set1.count(&3), 1);
        assert_eq!(set2, MultiSet::from(vec![2, 2, 3]));
    }

    #[test]
    fn test_pop_first_last() {
        let mut set: MultiSet<i64> = vec![1, 1, 2, 3, 3].into_iter().collect();

        assert_eq!(set.pop_first(), Some(1));
        assert_eq!(set.size(), 4);
        assert_eq!(set.count(&1), 1);

        assert_eq!(set.pop_last(), Some(3));
        assert_eq!(set.size(), 3);
        assert_eq!(set.count(&3), 1);

        assert_eq!(set.pop_last(), Some(3));
        assert_eq!(set.size(), 2);
        assert_eq!(set.count(&3), 0);

        assert_eq!(set.pop_first(), Some(1));
        assert_eq!(set.pop_first(), Some(2));

        assert_eq!(set.pop_first(), None);
        assert_eq!(set.pop_last(), None);
        assert!(set.is_empty());
    }

    #[test]
    fn test_remove() {
        let mut set: MultiSet<i32> = vec![1, 1, 2].into_iter().collect();

        assert!(set.remove(&1));
        assert_eq!(set.size(), 2);
        assert_eq!(set.len(), 2);
        assert_eq!(set.count(&1), 1);

        assert!(set.remove(&1));
        assert_eq!(set.size(), 1);
        assert_eq!(set.len(), 1);
        assert!(!set.contains(&1));
        assert_eq!(set.count(&1), 0);

        assert!(!set.remove(&3));
        assert_eq!(set.size(), 1);
        assert_eq!(set, MultiSet::from(vec![2]));
    }

    #[test]
    fn test_lower_bound() {
        let set: MultiSet<i32> = vec![1, 2, 2, 4].into_iter().collect();

        assert_eq!(set.lower_bound(Included(&0)), Some(&1));
        assert_eq!(set.lower_bound(Included(&2)), Some(&2));
        assert_eq!(set.lower_bound(Excluded(&2)), Some(&4));
        assert_eq!(set.lower_bound(Included(&5)), None);
    }

    #[test]
    fn test_upper_bound() {
        let set: MultiSet<i32> = vec![1, 2, 2, 4].into_iter().collect();

        assert_eq!(set.upper_bound(Included(&4)), Some(&4));
        assert_eq!(set.upper_bound(Excluded(&4)), Some(&2));
        assert_eq!(set.upper_bound(Included(&2)), Some(&2));
        assert_eq!(set.upper_bound(Excluded(&1)), None);
    }

    #[test]
    fn test_iter() {
        let set: MultiSet<i32> = vec![3, 1, 2, 2].into_iter().collect();

        let values: Vec<i32> = set.iter().copied().collect();
        assert_eq!(values, vec![1, 2, 2, 3]);

        let values_rev: Vec<i32> = set.iter().rev().copied().collect();
        assert_eq!(values_rev, vec![3, 2, 2, 1]);
    }

    #[test]
    fn test_range() {
        let set: MultiSet<i32> = vec![1, 2, 2, 3, 4, 4, 5].into_iter().collect();

        let values: Vec<i32> = set.range(2..=4).copied().collect();
        assert_eq!(values, vec![2, 2, 3, 4, 4]);

        let values_rev: Vec<i32> = set.range(2..=4).rev().copied().collect();
        assert_eq!(values_rev, vec![4, 4, 3, 2, 2]);

        let excluded: Vec<i32> = set.range((Excluded(2), Excluded(5))).copied().collect();
        assert_eq!(excluded, vec![3, 4, 4]);
    }
}
