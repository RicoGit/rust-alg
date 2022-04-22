//! 94. Binary Tree Inorder Traversal

// Definition for a binary tree node.
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

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        Self::in_order(&root, &mut res);
        res
    }

    fn in_order(node: &Node, res: &mut Vec<i32>) {
        match node {
            None => (),
            Some(node) => {
                let node = node.borrow();
                Self::in_order(&node.left, res);
                res.push(node.val);
                Self::in_order(&node.right, res);
            }
        }
    }
}

struct Solution;
