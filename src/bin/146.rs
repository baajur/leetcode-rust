use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
type Node = Rc<RefCell<DoubleLinkedListNode>>;

#[derive(Debug)]
struct DoubleLinkedListNode {
    // this would cause cyclic reference??
    prev: Option<Node>,
    next: Option<Node>,
}

struct LRUCache {
    values: HashMap<i32, i32>,
    positions: HashMap<i32, DoubleLinkedListNode>,
    capacity: i32,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            values: HashMap::new(),
            positions: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.values.get(&key) {
            Some(&v) => {
                let node = self.positions.remove(&key).unwrap();
                match node.prev {
                    Some(prev) => {
                        prev.borrow_mut().next = node.next.clone();
                        if let Some(next) = node.next.clone() {
                            next.borrow_mut().prev = Some(prev);
                        }
                    }
                    None => {
                        if let Some(next) = node.next {
                            next.borrow_mut().prev = None;
                        }
                    }
                };
                v
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        ()
    }
}

fn main() {
    ()
}
