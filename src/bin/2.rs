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

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ans_root = Box::new(ListNode::new(0));
        let mut ans_ptr = &mut ans_root;
        let mut carry_up = 0;
        let mut ptr1 = l1.as_ref();
        let mut ptr2 = l2.as_ref();

        loop {
            let val1 = ptr1.map(|node| node.val).unwrap_or(0);
            let val2 = ptr2.map(|node| node.val).unwrap_or(0);
            let sum = val1 + val2 + carry_up;
            if sum <= 9 {
                ans_ptr.val = sum;
                carry_up = 0;
            } else {
                ans_ptr.val = sum % 10;
                carry_up = sum / 10;
            }

            ptr1 = ptr1.and_then(|p| p.next.as_ref());
            ptr2 = ptr2.and_then(|p| p.next.as_ref());

            if ptr1.is_none() && ptr2.is_none() && carry_up == 0 {
                break;
            }
            let next_node = Box::new(ListNode::new(0));
            ans_ptr.next = Some(next_node);
            ans_ptr = ans_ptr.next.as_mut().unwrap();
        }

        Some(ans_root)
    }

    pub fn recursive_add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::solve(l1.as_ref(), l2.as_ref(), 0)
    }

    fn solve(
        node1: Option<&Box<ListNode>>,
        node2: Option<&Box<ListNode>>,
        carry_up: i32,
    ) -> Option<Box<ListNode>> {
        if node1.is_none() && node2.is_none() && carry_up == 0 {
            return None;
        }
        let mut node = Box::new(ListNode::new(0));
        let val1 = node1.map(|n| n.val).unwrap_or(0);
        let val2 = node2.map(|n| n.val).unwrap_or(0);
        let sum = val1 + val2 + carry_up;
        node.val = sum % 10;
        node.next = Solution::solve(
            node1.and_then(|n| n.next.as_ref()),
            node2.and_then(|n| n.next.as_ref()),
            sum / 10,
        );
        Some(node)
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = {
            let mut root = Box::new(ListNode::new(2));
            root.next = Some(Box::new(ListNode::new(4)));
            Some(root)
        };
        let l2 = {
            let mut root = Box::new(ListNode::new(5));
            root.next = Some(Box::new(ListNode::new(6)));
            Some(root)
        };
        let expected = {
            let mut root = Box::new(ListNode::new(7));
            root.next = {
                let mut second = Box::new(ListNode::new(0));
                second.next = Some(Box::new(ListNode::new(1)));
                Some(second)
            };
            Some(root)
        };
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_recursive_add_two_numbers() {
        let l1 = {
            let mut root = Box::new(ListNode::new(2));
            root.next = Some(Box::new(ListNode::new(4)));
            Some(root)
        };
        let l2 = {
            let mut root = Box::new(ListNode::new(5));
            root.next = Some(Box::new(ListNode::new(6)));
            Some(root)
        };
        let expected = {
            let mut root = Box::new(ListNode::new(7));
            root.next = {
                let mut second = Box::new(ListNode::new(0));
                second.next = Some(Box::new(ListNode::new(1)));
                Some(second)
            };
            Some(root)
        };
        assert_eq!(Solution::recursive_add_two_numbers(l1, l2), expected);
    }
}
