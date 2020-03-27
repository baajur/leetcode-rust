#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

use std::collections::VecDeque;

struct NestedIterator {
    unvisited: VecDeque<i32>,
}

impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut it = Self {
            unvisited: VecDeque::new(),
        };
        it.push_all(nestedList);
        it
    }

    fn push_all(&mut self, values: Vec<NestedInteger>) {
        for v in values {
            match v {
                NestedInteger::Int(x) => self.unvisited.push_back(x),
                NestedInteger::List(vec) => self.push_all(vec),
            }
        }
    }

    // panics: when all elements have already been iterated
    fn next(&mut self) -> i32 {
        self.unvisited.pop_front().unwrap()
    }

    fn has_next(&self) -> bool {
        !self.unvisited.is_empty()
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! nested_integer {
        ([ $( $t:tt ),* ]) => {
            NestedInteger::List(vec![ $( nested_integer![$t] ),* ])
        };
        ( $n:tt ) => {
            NestedInteger::Int($n)
        };
    }

    fn wrap(ni: NestedInteger) -> Vec<NestedInteger> {
        vec![ni]
    }

    #[test]
    fn test_nested_iterator() {
        let mut it = NestedIterator::new(wrap(nested_integer!([[1, 2], 3, [4, [5, [6, 7]]]])));
        assert!(it.has_next());
        assert_eq!(it.next(), 1);
        assert!(it.has_next());
        assert_eq!(it.next(), 2);
        assert!(it.has_next());
        assert_eq!(it.next(), 3);
        assert!(it.has_next());
        assert_eq!(it.next(), 4);
        assert!(it.has_next());
        assert_eq!(it.next(), 5);
        assert!(it.has_next());
        assert_eq!(it.next(), 6);
        assert!(it.has_next());
        assert_eq!(it.next(), 7);
        assert!(!it.has_next());

        let mut it = NestedIterator::new(wrap(nested_integer!([])));
        assert!(!it.has_next());

        let mut it = NestedIterator::new(wrap(nested_integer!([[[[[[[[[[[[4]]]]]], 8]]]]]])));
        assert!(it.has_next());
        assert_eq!(it.next(), 4);
        assert!(it.has_next());
        assert_eq!(it.next(), 8);
        assert!(!it.has_next());
    }
}
