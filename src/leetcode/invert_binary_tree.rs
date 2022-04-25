//! 226. Invert Binary Tree

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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&root);
        root
    }

    fn helper(node: &Node) {
        if let Some(node_cell) = node {
            let mut node = node_cell.borrow_mut();
            Self::helper(&node.left);
            Self::helper(&node.right);

            unsafe {
                // deceive borrow checker
                let right_prt: *mut Node = &mut node.right;
                std::mem::swap(&mut node.left, &mut (*right_prt) );
            }

            // let tmp = node.right.take();
            // node.right = node.left.take();
            // node.left = tmp;
        }
    }
}

struct Solution;
