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

struct Solution;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() || preorder.len() != inorder.len() {
            return None;
        }
        let mut preorder: VecDeque<_> = preorder.into();
        dfs(&mut preorder, &inorder)
    }
}

fn dfs(preorder: &mut VecDeque<i32>, inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() || inorder.is_empty() {
        return None;
    }

    let node_val = preorder.pop_front().unwrap();
    let val_index = inorder.iter().position(|&v| v == node_val).unwrap();

    Some(Rc::new(RefCell::new(TreeNode {
        val: node_val,
        left: if val_index == 0 {
            None
        } else {
            dfs(preorder, &inorder[0..val_index])
        },
        right: if val_index == inorder.len() - 1 {
            None
        } else {
            dfs(preorder, &inorder[val_index + 1..])
        },
    })))
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
    fn test_build_tree() {
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
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            root
        );

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
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15], vec![9, 3, 15, 20]),
            root
        );
    }
}
