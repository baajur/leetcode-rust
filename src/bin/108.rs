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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        dfs(&nums)
    }
}

fn dfs(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }
    // the value of root node is the center of nums
    let center = nums.len() / 2;
    Some(Rc::new(RefCell::new(TreeNode {
        val: nums[center],
        left: if center == 0 {
            None
        } else {
            dfs(&nums[0..center])
        },
        right: if center == nums.len() - 1 {
            None
        } else {
            dfs(&nums[(center + 1)..])
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
        //      9
        //    /  \
        //   7    20
        //  / \  /  \
        // 3  * 15   *
        let tree_node = TreeNode {
            val: 9,
            left: make_node_with(7, make_node(3), None),
            right: make_node_with(20, make_node(15), None),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert_eq!(Solution::sorted_array_to_bst(vec![3, 7, 9, 15, 20]), root);

        //      0
        //    /  \
        //   -3    9
        //  / \   /  \
        // -10 * 5     *
        let tree_node = TreeNode {
            val: 0,
            left: make_node_with(-3, make_node(-10), None),
            right: make_node_with(9, make_node(5), None),
        };
        let root = Some(Rc::new(RefCell::new(tree_node)));
        assert_eq!(Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]), root);
    }
}
