use leetcode_rust::tree_node::*;

struct Solution;

impl Solution {
    pub fn right_side_view(root: Node) -> Vec<i32> {
        // traverse binary tree inorder
        let mut ans = Vec::new();
        dfs(root, &mut ans, 0);

        ans
    }
}

fn dfs(node: Node, vec: &mut Vec<i32>, depth: usize) {
    if node.is_none() {
        return;
    }
    let node = node.unwrap();
    if vec.len() == depth {
        vec.push(-1); // update later
    }

    dfs(node.borrow_mut().left.take(), vec, depth + 1);
    vec[depth] = node.borrow().val;
    dfs(node.borrow_mut().right.take(), vec, depth + 1);
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_side_view() {
        //     1
        //    / \
        //   2   3
        //    \   \
        //     5   4
        let tree = make_node_with(
            1,
            make_node_with(2, None, make_node(5)),
            make_node_with(3, None, make_node(4)),
        );
        assert_eq!(Solution::right_side_view(tree), vec![1, 3, 4]);

        //     1
        //    / \
        //   2   3
        //    \
        //     5
        let tree = make_node_with(1, make_node_with(2, None, make_node(5)), make_node(3));
        assert_eq!(Solution::right_side_view(tree), vec![1, 3, 5]);

        //     1
        //    /
        //   2
        //    \
        //     5
        let tree = make_node_with(1, make_node_with(2, None, make_node(5)), None);
        assert_eq!(Solution::right_side_view(tree), vec![1, 2, 5]);

        //     1
        //    /
        //   2
        let tree = make_node_with(1, make_node(2), None);
        assert_eq!(Solution::right_side_view(tree), vec![1, 2]);

        //     1
        let tree = make_node(1);
        assert_eq!(Solution::right_side_view(tree), vec![1]);
    }
}
