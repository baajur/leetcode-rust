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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut values = Vec::new();
        traverse(&mut values, root);
        build_balance_bst(&values)
    }
}

fn traverse(vec: &mut Vec<i32>, mut node: Option<Rc<RefCell<TreeNode>>>) {
    if node.is_none() {
        return;
    }
    let val = node.as_ref().unwrap().borrow().val;
    let left = node.as_mut().unwrap().borrow_mut().left.take();
    let right = node.as_mut().unwrap().borrow_mut().right.take();
    traverse(vec, left);
    vec.push(val);
    traverse(vec, right);
}

fn build_balance_bst(values: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() {
        return None;
    }
    let center = values.len() / 2;
    Some(Rc::new(RefCell::new(TreeNode {
        val: values[center],
        left: build_balance_bst(&values[0..center]),
        right: build_balance_bst(&values[(center + 1)..]),
    })))
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

fn main() {
    let node = make_node_with(
        1,
        None,
        make_node_with(2, None, make_node_with(3, None, make_node(4))),
    );
    Solution::balance_bst(node);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_bst() {
        let root = make_node_with(
            1,
            None,
            make_node_with(2, None, make_node_with(3, None, make_node(4))),
        );
        let expected = make_node_with(3, make_node_with(2, make_node(1), None), make_node(4));
        assert_eq!(Solution::balance_bst(root), expected);
    }
}
