use leetcode_rust::tree_node::*;
use std::cell::RefCell;
use std::rc::Rc;

pub type Node = Option<Rc<RefCell<TreeNode>>>;
struct Solution;

impl Solution {
    pub fn merge_trees(t1: Node, t2: Node) -> Node {
        dfs(t1, t2)
    }
}

fn dfs(mut n1: Node, mut n2: Node) -> Node {
    if n1.is_none() && n2.is_none() {
        return None;
    }

    if n1.is_none() {
        std::mem::swap(&mut n1, &mut n2);
    }
    // n1 is absolutely some,
    let mut n1 = n1.unwrap();

    Some(Rc::new(RefCell::new(match n2 {
        Some(mut n2) => {
            let l1 = n1.borrow_mut().left.take();
            let l2 = n2.borrow_mut().left.take();
            let r1 = n1.borrow_mut().right.take();
            let r2 = n2.borrow_mut().right.take();
            TreeNode {
                val: n1.borrow().val + n2.borrow().val,
                left: dfs(l1, l2),
                right: dfs(r1, r2),
            }
        }
        None => {
            let l = n1.borrow_mut().left.take();
            let r = n1.borrow_mut().right.take();
            TreeNode {
                val: n1.borrow().val,
                left: dfs(l, None),
                right: dfs(r, None),
            }
        }
    })))
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_trees() {
        let tree1 = make_node_with(1, make_node_with(3, make_node(5), None), make_node(2));
        let tree2 = make_node_with(
            2,
            make_node_with(1, None, make_node(4)),
            make_node_with(3, None, make_node(7)),
        );
        let expected = make_node_with(
            3,
            make_node_with(4, make_node(5), make_node(4)),
            make_node_with(5, None, make_node(7)),
        );
        assert_eq!(Solution::merge_trees(tree1, tree2), expected);

        let tree1 = make_node(1);
        let tree2 = None;
        let expected = make_node(1);
        assert_eq!(Solution::merge_trees(tree1, tree2), expected);

        let tree1 = None;
        let tree2 = None;
        let expected = None;
        assert_eq!(Solution::merge_trees(tree1, tree2), expected);
    }
}
