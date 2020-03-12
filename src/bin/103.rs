#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut ret = Vec::new();
        ret.push({
            let mut t = VecDeque::new();
            t.push_back(root.as_ref().unwrap().borrow().val);
            t
        });
        queue.push_back((root.unwrap(), 0));

        while !queue.is_empty() {
            let (node, level) = queue.pop_front().unwrap();
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            let level = level + 1;

            if left.is_none() && right.is_none() {
                continue;
            }

            if ret.len() <= level {
                ret.push(VecDeque::new());
            }

            if let Some(l) = left {
                if level % 2 == 0 {
                    ret[level].push_back(l.borrow().val);
                } else {
                    ret[level].push_front(l.borrow().val);
                }
                queue.push_back((l, level));
            }
            if let Some(r) = right {
                if level % 2 == 0 {
                    ret[level].push_back(r.borrow().val);
                } else {
                    ret[level].push_front(r.borrow().val);
                }
                queue.push_back((r, level));
            }
        }

        ret.into_iter().map(|r| r.into()).collect()
    }
}

fn main() {
    ()
}

fn make_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

fn make_node_with(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_order() {
        //   2
        //  / \
        // 1   3
        let tree_node = TreeNode {
            val: 2,
            left: make_node(1),
            right: make_node(3),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert_eq!(
            Solution::zigzag_level_order(root),
            vec![vec![2], vec![3, 1]]
        );

        //   4
        //  / \
        // 1   3
        let tree_node = TreeNode {
            val: 4,
            left: make_node(1),
            right: make_node(3),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert_eq!(
            Solution::zigzag_level_order(root),
            vec![vec![4], vec![3, 1]]
        );

        //   1
        //  / \
        // 1   *
        let tree_node = TreeNode {
            val: 1,
            left: make_node(1),
            right: None,
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert_eq!(Solution::zigzag_level_order(root), vec![vec![1], vec![1]]);

        //   1
        //  / \
        // *   3
        let tree_node = TreeNode {
            val: 1,
            left: None,
            right: make_node(3),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert_eq!(Solution::zigzag_level_order(root), vec![vec![1], vec![3]]);

        //   10
        //  / \
        // 5   15
        //    / \
        //   6  20
        let tree_node = TreeNode {
            val: 10,
            left: make_node(5),
            right: make_node_with(15, make_node(6), make_node(20)),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert_eq!(
            Solution::zigzag_level_order(root),
            vec![vec![10], vec![15, 5], vec![6, 20]]
        );

        // 2147483647
        let tree_node = TreeNode {
            val: 2147483647,
            left: None,
            right: None,
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert_eq!(Solution::zigzag_level_order(root), vec![vec![2147483647]]);

        // *
        let root = None;
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::zigzag_level_order(root), expected);
    }
}
