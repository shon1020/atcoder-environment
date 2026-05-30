use cargo_snippet::snippet;

#[snippet(name = "queue")]
pub mod queue {
    use std::collections::{vec_deque, VecDeque};

    // 入れた順に取り出すFIFOキュー（VecDequeに委譲する薄いラッパー）
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Queue<T> {
        data: VecDeque<T>,
    }

    impl<T> Default for Queue<T> {
        // 空のキューを既定値として返す
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> From<Vec<T>> for Queue<T> {
        // Vecの先頭を先頭としてキューに変換する
        fn from(vec: Vec<T>) -> Self {
            Self {
                data: VecDeque::from(vec),
            }
        }
    }

    impl<T> Queue<T> {
        // 空のキューを生成する
        pub fn new() -> Self {
            Self {
                data: VecDeque::new(),
            }
        }

        // 容量を確保して生成する
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                data: VecDeque::with_capacity(capacity),
            }
        }

        // 末尾に追加する
        pub fn push(&mut self, value: T) {
            self.data.push_back(value);
        }

        // 先頭を取り出す（空ならNone）
        pub fn pop(&mut self) -> Option<T> {
            self.data.pop_front()
        }

        // 先頭を覗く（空ならNone）
        pub fn front(&self) -> Option<&T> {
            self.data.front()
        }

        // 末尾を覗く（空ならNone）
        pub fn back(&self) -> Option<&T> {
            self.data.back()
        }

        // 要素数を返す
        pub fn len(&self) -> usize {
            self.data.len()
        }

        // 空か確認する
        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        // 全要素を削除する
        pub fn clear(&mut self) {
            self.data.clear();
        }

        // 先頭から末尾への参照イテレータを返す
        pub fn iter(&self) -> vec_deque::Iter<'_, T> {
            self.data.iter()
        }
    }

    impl<T> FromIterator<T> for Queue<T> {
        // イテレータの要素を取り出し順にキューへ詰める
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Queue<T> {
            Self {
                data: iter.into_iter().collect(),
            }
        }
    }

    // 所有権ごと先頭から消費するイテレータ
    impl<T> IntoIterator for Queue<T> {
        type Item = T;
        type IntoIter = vec_deque::IntoIter<T>;
        // 先頭から末尾の順で所有権ごと要素を返す
        fn into_iter(self) -> Self::IntoIter {
            self.data.into_iter()
        }
    }

    // 参照で先頭から走査するイテレータ（for x in &queue 用）
    impl<'a, T> IntoIterator for &'a Queue<T> {
        type Item = &'a T;
        type IntoIter = vec_deque::Iter<'a, T>;
        // 先頭から末尾の順で参照を返す
        fn into_iter(self) -> Self::IntoIter {
            self.data.iter()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::queue::Queue;

    #[test]
    fn test_new() {
        let queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_default() {
        let queue: Queue<i32> = Queue::default();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_from() {
        let queue: Queue<i32> = Queue::from(vec![1, 2, 3]);
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.front(), Some(&1));
        assert_eq!(queue.back(), Some(&3));
    }

    #[test]
    fn test_from_iter() {
        let queue: Queue<i32> = (1..=3).collect();
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.front(), Some(&1));
        assert_eq!(queue.back(), Some(&3));
    }

    #[test]
    fn test_fifo_order() {
        let mut queue: Queue<i32> = Queue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_empty_peek() {
        let mut queue: Queue<i32> = Queue::new();
        assert_eq!(queue.front(), None);
        assert_eq!(queue.back(), None);
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_clear() {
        let mut queue: Queue<i32> = Queue::from(vec![1, 2, 3]);
        queue.clear();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_non_copy_type() {
        let mut queue: Queue<String> = Queue::new();
        queue.push(String::from("a"));
        queue.push(String::from("b"));

        assert_eq!(queue.front(), Some(&String::from("a")));
        assert_eq!(queue.pop(), Some(String::from("a")));
        assert_eq!(queue.pop(), Some(String::from("b")));
    }

    #[test]
    fn test_into_iter_owned() {
        let queue: Queue<i32> = Queue::from(vec![1, 2, 3]);
        let collected: Vec<i32> = queue.into_iter().collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }

    #[test]
    fn test_into_iter_ref() {
        let queue: Queue<i32> = Queue::from(vec![1, 2, 3]);
        let collected: Vec<i32> = (&queue).into_iter().copied().collect();
        assert_eq!(collected, vec![1, 2, 3]);
        // 参照走査後もキューは消費されない
        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn test_iter() {
        let queue: Queue<i32> = Queue::from(vec![1, 2, 3]);
        let collected: Vec<i32> = queue.iter().copied().collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }
}
