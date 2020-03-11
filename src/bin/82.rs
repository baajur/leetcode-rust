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

struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut to_be_removed = head.as_ref().unwrap().val - 1;
        let mut dummy = Some(Box::new(ListNode {
            next: head,
            val: to_be_removed,
        }));
        let mut ptr = &mut dummy.as_mut().unwrap().next;

        loop {
            match ptr {
                None => return dummy.unwrap().next,
                Some(p) if p.val == to_be_removed => *ptr = p.next.take(),
                Some(p) if p.next.is_some() && p.val == p.next.as_ref().unwrap().val => {
                    to_be_removed = p.val
                }
                Some(p) => {
                    ptr = &mut p.next;
                    if let Some(q) = ptr {
                        to_be_removed = q.val - 1;
                    }
                }
            }
        }
    }
}

fn main() {
    Solution::delete_duplicates(make_list(vec![1, 2, 3, 3, 4, 4, 5]));
}

fn make_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    if vec.is_empty() {
        return None;
    }
    let mut dummy = Box::new(ListNode::new(-1));
    let mut ptr = &mut dummy;

    for v in vec {
        let node = Box::new(ListNode::new(v));
        ptr.next = Some(node);
        ptr = ptr.next.as_mut().unwrap();
    }

    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_duplicates() {
        assert_eq!(
            Solution::delete_duplicates(make_list(vec![1, 2, 3, 3, 4, 4, 5])),
            make_list(vec![1, 2, 5])
        );
        assert_eq!(
            Solution::delete_duplicates(make_list(vec![1, 1, 2, 2])),
            make_list(vec![])
        );
        assert_eq!(
            Solution::delete_duplicates(make_list(vec![1, 1, 2, 2, 3, 4, 4])),
            make_list(vec![3])
        );
    }
}
