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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // breadth first search
        if root.is_none() {
            return 0;
        }
        use std::collections::VecDeque;
        let mut q = VecDeque::new();
        q.push_back((root, 1));

        while !q.is_empty() {
            let (node, depth) = q.pop_front().unwrap();
            match node {
                None => continue,
                Some(n) => {
                    let left = n.borrow_mut().left.take();
                    let right = n.borrow_mut().right.take();
                    if left.is_none() && right.is_none() {
                        return depth;
                    }
                    q.push_back((left, depth + 1));
                    q.push_back((right, depth + 1));
                }
            }
        }

        unreachable!();
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
    fn test_min_depth() {
        //   2
        //  / \
        // 1   3
        let tree_node = TreeNode {
            val: 2,
            left: make_node(1),
            right: make_node(3),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert_eq!(Solution::min_depth(root), 2);

        //   4
        //  / \
        // 1   3
        let tree_node = TreeNode {
            val: 4,
            left: make_node(1),
            right: make_node(3),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert_eq!(Solution::min_depth(root), 2);

        //   1
        //  / \
        // 1   *
        let tree_node = TreeNode {
            val: 1,
            left: make_node(1),
            right: None,
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert_eq!(Solution::min_depth(root), 2);

        //   1
        //  / \
        // *   3
        let tree_node = TreeNode {
            val: 1,
            left: None,
            right: make_node(3),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert_eq!(Solution::min_depth(root), 2);

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
        assert_eq!(Solution::min_depth(root), 2);

        // 2147483647
        let tree_node = TreeNode {
            val: 2147483647,
            left: None,
            right: None,
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert_eq!(Solution::min_depth(root), 1);

        // *
        let root = None;
        assert_eq!(Solution::min_depth(root), 0);
    }
}
