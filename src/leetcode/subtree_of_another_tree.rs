//! 572. Subtree of Another Tree

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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::helper(&root, &sub_root)
    }

    fn helper(root: &Node, sub_root: &Node) -> bool {
        if Self::compare(root, sub_root) {
            return true
        }

        if let Some(node) = root {
            let node = node.borrow();
            Self::helper(&node.left, sub_root) || Self::helper(&node.right, sub_root)
        } else {
            return false
        }
    }

    fn compare(root: &Node, sub_root: &Node) -> bool {
        match (root, sub_root) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(node), None) => false,
            (Some(node_cell1), Some(node_cell2)) => {
                let node1 = node_cell1.borrow();
                let node2 = node_cell2.borrow();
                node1.val == node2.val && Self::compare(&node1.left, &node2.left) && Self::compare(&node1.right, &node2.right)
            }
        }
    }

    // comparing 2 subtrees does by compiler (see #[derive(Debug, PartialEq, Eq)])
    pub fn is_subtree_chet(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {

        if root == sub_root {return true}
        if let Some(node) = root {
            let node = node.borrow();
            Self::is_subtree(node.left.clone(), sub_root.clone()) ||
                Self::is_subtree(node.right.clone(), sub_root.clone())
        } else {
            return false
        }
    }
}

struct Solution;
