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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        dfs(root.as_ref(), None, None)
    }
}

fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, lower: Option<i32>, upper: Option<i32>) -> bool {
    if node.is_none() {
        return true;
    }
    let node = node.unwrap();
    let val = node.borrow().val;
    if lower.filter(|&l| val <= l).is_some() || upper.filter(|&u| val >= u).is_some() {
        return false;
    }

    dfs(node.borrow().left.as_ref(), lower, Some(val))
        && dfs(node.borrow().right.as_ref(), Some(val), upper)
}

fn main() {
    Solution::is_valid_bst(Some(Rc::new(RefCell::new(TreeNode::new(5)))));
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
    fn test_is_valid_bst() {
        //   2
        //  / \
        // 1   3
        let tree_node = TreeNode {
            val: 2,
            left: make_node(1),
            right: make_node(3),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert!(Solution::is_valid_bst(root));

        //   4
        //  / \
        // 1   3
        let tree_node = TreeNode {
            val: 4,
            left: make_node(1),
            right: make_node(3),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert!(!Solution::is_valid_bst(root));

        //   4
        //  / \
        // 8   5
        let tree_node = TreeNode {
            val: 4,
            left: make_node(8),
            right: make_node(5),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert!(!Solution::is_valid_bst(root));

        //   1
        //  / \
        // 1   *
        let tree_node = TreeNode {
            val: 1,
            left: make_node(1),
            right: None,
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert!(!Solution::is_valid_bst(root));

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
        assert!(!Solution::is_valid_bst(root));

        //   10
        //  / \
        // 5   15
        //    / \
        //  11  20
        let tree_node = TreeNode {
            val: 10,
            left: make_node(5),
            right: make_node_with(15, make_node(11), make_node(20)),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert!(Solution::is_valid_bst(root));

        // 2147483647
        let tree_node = TreeNode {
            val: 2147483647,
            left: None,
            right: None,
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert!(Solution::is_valid_bst(root));
    }
}
