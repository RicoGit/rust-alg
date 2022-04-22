//! 538. Convert BST to Greater Tree

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
use std::ops::DerefMut;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&root, &mut 0);
        root
    }

    fn helper(node: &Node, sum: &mut i32) {
        match node {
            None => (),
            Some(node_cell) => {
                let mut node = node_cell.borrow_mut();
                Self::helper(&node.right, sum); // first go to the right in depth
                *sum += node.val;
                node.val = *sum;
                Self::helper(&node.left, sum);
            }
        };
    }
}

struct Solution;
