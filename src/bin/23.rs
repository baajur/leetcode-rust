// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &ListNode) -> Ordering {
        other.val.cmp(&self.val)
    }
}

struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for node in lists {
            if node.is_some() {
                heap.push(node.unwrap());
            }
        }
        let mut head = heap.pop()?;
        let mut pointer = &mut head;
        while !heap.is_empty() {
            if pointer.next.is_some() {
                heap.push(pointer.next.take()?);
            }
            pointer.next = Some(heap.pop()?);
            pointer = pointer.next.as_mut()?;
        }

        Some(head)
    }
}

fn main() {
    let lists = {
        let mut v = Vec::new();
        v.push(Some(Box::new(ListNode::new(3))));
        v.push(Some(Box::new(ListNode::new(1))));
        v.push(None);
        v.push(None);
        v.push(None);
        v
    };
    Solution::merge_k_lists(lists);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_l_lists() {}
}
