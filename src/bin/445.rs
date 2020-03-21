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
        let mut val1 = Vec::new();
        let mut val2 = Vec::new();

        traverse(l1.as_ref(), &mut val1);
        traverse(l2.as_ref(), &mut val2);
        val1.reverse();
        val2.reverse();

        use std::collections::VecDeque;
        let mut digits = VecDeque::new();
        let mut carry = 0;
        for i in 0..std::cmp::max(val1.len(), val2.len()) {
            let &x = val1.get(i).unwrap_or(&0);
            let &y = val2.get(i).unwrap_or(&0);
            let s = x + y + carry;
            digits.push_front(s % 10);
            carry = s / 10;
        }
        if carry > 0 {
            digits.push_front(carry);
        }
        let digits: Vec<i32> = digits.into();

        make_linked_list(0, &digits)
    }
}

fn make_linked_list(index: usize, digits: &[i32]) -> Option<Box<ListNode>> {
    if index >= digits.len() {
        None
    } else {
        Some(Box::new(ListNode {
            val: digits[index],
            next: make_linked_list(index + 1, digits),
        }))
    }
}

fn traverse(node: Option<&Box<ListNode>>, val: &mut Vec<i32>) {
    match node {
        None => (),
        Some(ref n) => {
            val.push(n.val);
            traverse(n.next.as_ref(), val);
        }
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
        let l1 = make_linked_list(0, &[7, 2, 4, 3]);
        let l2 = make_linked_list(0, &[5, 6, 4]);
        let expected = make_linked_list(0, &[7, 8, 0, 7]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);

        let l1 = make_linked_list(0, &[7, 2, 4, 3]);
        let l2 = make_linked_list(0, &[0]);
        let expected = make_linked_list(0, &[7, 2, 4, 3]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);

        let l1 = make_linked_list(0, &[2, 4]);
        let l2 = make_linked_list(0, &[1, 0, 0]);
        let expected = make_linked_list(0, &[1, 2, 4]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);

        let l1 = make_linked_list(0, &[0]);
        let l2 = make_linked_list(0, &[0]);
        let expected = make_linked_list(0, &[0]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);

        let l1 = make_linked_list(0, &[3, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
        let l2 = make_linked_list(0, &[7]);
        let expected = make_linked_list(0, &[4, 0, 0, 0, 0, 0, 0, 0, 0, 6]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);

        let l1 = make_linked_list(0, &[3, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
        let l2 = make_linked_list(0, &[7]);
        let expected = make_linked_list(0, &[4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);

        let l1 = make_linked_list(0, &[5]);
        let l2 = make_linked_list(0, &[7]);
        let expected = make_linked_list(0, &[1, 2]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }
}
