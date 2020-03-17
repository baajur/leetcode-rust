use leetcode_rust::tree_node::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn split_bst(root: Node, v: i32) -> Vec<Node> {
        if root.is_none() {
            return vec![None, None];
        }
        let mut root = root.unwrap();
        if root.borrow().val <= v {
            let right = root.borrow_mut().right.take();
            let mut bns = Solution::split_bst(right, v);
            root.borrow_mut().right = bns.swap_remove(0);
            vec![Some(root), bns.swap_remove(0)]
        } else {
            let left = root.borrow_mut().left.take();
            let mut bns = Solution::split_bst(left, v);
            root.borrow_mut().left = bns.swap_remove(1);
            vec![bns.swap_remove(0), Some(root)]
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
    fn test_split_bst() {
        assert_eq!(Solution::split_bst(None, 4), vec![None, None]);

        let root = make_node_with(
            4,
            make_node_with(2, make_node(1), make_node(3)),
            make_node_with(6, make_node(5), make_node(7)),
        );
        let expected_1st = make_node_with(2, make_node(1), None);
        let expected_2nd = make_node_with(
            4,
            make_node(3),
            make_node_with(6, make_node(5), make_node(7)),
        );
        assert_eq!(
            Solution::split_bst(root, 2),
            vec![expected_1st, expected_2nd]
        );
    }
}
