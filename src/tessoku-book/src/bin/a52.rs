use proconio::input;

define_queries! {
    enum Query: usize {
        1 => Push { s: String },
        2 => Answer{},
        3 => Out {},
    }
}

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      q: usize,
      queries: [Query; q],
    }

    let mut queue = queue::Queue::new();

    for query in queries {
        match query {
            Query::Push { s } => {
                queue.push(s);
            }
            Query::Answer {} => {
                println!("{}", queue.front().unwrap());
            }
            Query::Out {} => {
                queue.pop();
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
pub mod queue {
    use std::collections::{vec_deque, VecDeque};
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Queue<T> {
        data: VecDeque<T>,
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
    impl<T> From<Vec<T>> for Queue<T> {
        fn from(vec: Vec<T>) -> Self {
            Self {
                data: VecDeque::from(vec),
            }
        }
    }
    impl<T> Queue<T> {
        pub fn new() -> Self {
            Self {
                data: VecDeque::new(),
            }
        }
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                data: VecDeque::with_capacity(capacity),
            }
        }
        pub fn push(&mut self, value: T) {
            self.data.push_back(value);
        }
        pub fn pop(&mut self) -> Option<T> {
            self.data.pop_front()
        }
        pub fn front(&self) -> Option<&T> {
            self.data.front()
        }
        pub fn back(&self) -> Option<&T> {
            self.data.back()
        }
        pub fn len(&self) -> usize {
            self.data.len()
        }
        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
        pub fn clear(&mut self) {
            self.data.clear();
        }
        pub fn iter(&self) -> vec_deque::Iter<'_, T> {
            self.data.iter()
        }
    }
    impl<T> FromIterator<T> for Queue<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Queue<T> {
            Self {
                data: iter.into_iter().collect(),
            }
        }
    }
    impl<T> IntoIterator for Queue<T> {
        type Item = T;
        type IntoIter = vec_deque::IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter {
            self.data.into_iter()
        }
    }
    impl<'a, T> IntoIterator for &'a Queue<T> {
        type Item = &'a T;
        type IntoIter = vec_deque::Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter {
            self.data.iter()
        }
    }
}
