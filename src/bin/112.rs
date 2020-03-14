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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        dfs(root, 0, sum)
    }
}

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, sum: i32, target: i32) -> bool {
    match node {
        None => false,
        Some(node) => {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let next_sum = sum + node.borrow().val;
            if left.is_none() && right.is_none() {
                next_sum == target
            } else {
                dfs(left, next_sum, target) || dfs(right, next_sum, target)
            }
        }
    }
}

fn main() {
    //    3
    //  / \
    // 9   20
    //    / \
    //   15  7
    let tree_node = TreeNode {
        val: 3,
        left: make_node(9),
        right: make_node_with(20, make_node(15), make_node(7)),
    };
    let root = Some(Rc::new(RefCell::new(tree_node)));
    assert!(Solution::has_path_sum(root.clone(), 38));
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
    fn test_has_path_sum() {
        //    3
        //  / \
        // 9   20
        //    / \
        //   15  7
        let tree_node = TreeNode {
            val: 3,
            left: make_node(9),
            right: make_node_with(20, make_node(15), make_node(7)),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert!(Solution::has_path_sum(root, 38));

        //    3
        //  / \
        // 9   20
        //    / \
        //   15  *
        let tree_node = TreeNode {
            val: 3,
            left: make_node(9),
            right: make_node_with(20, make_node(15), None),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert!(!Solution::has_path_sum(root, 23));

        //   2
        //  / \
        // 1   3
        let tree_node = TreeNode {
            val: 2,
            left: make_node(1),
            right: make_node(3),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert!(!Solution::has_path_sum(root, 2));

        //   4
        //  / \
        // 1   3
        let tree_node = TreeNode {
            val: 4,
            left: make_node(1),
            right: make_node(3),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert!(Solution::has_path_sum(root, 7));

        //   1
        //  / \
        // 1   *
        let tree_node = TreeNode {
            val: 1,
            left: make_node(1),
            right: None,
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert!(Solution::has_path_sum(root, 2));
        let tree_node = TreeNode {
            val: 1,
            left: make_node(1),
            right: None,
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert!(!Solution::has_path_sum(root, 1));
        let tree_node = TreeNode {
            val: 1,
            left: make_node(1),
            right: None,
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert!(!Solution::has_path_sum(root, 11));

        //   *
        assert!(!Solution::has_path_sum(None, 2));
    }
}
