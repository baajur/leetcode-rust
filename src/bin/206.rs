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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut cur = head;
        while let Some(node) = cur.as_mut() {
            let next_tmp = node.next.take();
            node.next = prev;
            prev = cur;
            cur = next_tmp;
        }
        prev
    }
}

fn main() {
    ()
}

fn make_linked_list(values: &[i32]) -> Option<Box<ListNode>> {
    if values.is_empty() {
        return None;
    }
    Some(Box::new(ListNode {
        val: values[0],
        next: make_linked_list(&values[1..]),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list() {
        assert_eq!(
            Solution::reverse_list(make_linked_list(&vec![1, 2, 3, 4, 5])),
            make_linked_list(&vec![5, 4, 3, 2, 1])
        );
        assert_eq!(
            Solution::reverse_list(make_linked_list(&vec![1])),
            make_linked_list(&vec![1])
        );
        assert_eq!(Solution::reverse_list(None), None);
    }
}
