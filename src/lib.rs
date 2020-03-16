pub mod tree_node {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub type Node = Option<Rc<RefCell<TreeNode>>>;
    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: Node,
        pub right: Node,
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

    pub fn make_node(val: i32) -> Node {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    pub fn make_node_with(val: i32, left: Node, right: Node) -> Node {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}
