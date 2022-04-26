//! 98. Validate Binary Search Tree


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

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::helper(&root, i64::MIN, i64::MAX)
    }

    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(cell) = root {
            let node = cell.borrow();
            let val = node.val as i64;
            // println!("{} {} {}", val , min, max);

            if val <= min || val >= max {
                return false
            }

            Self::helper(&node.left, min, val) && Self::helper(&node.right, val, max)
        } else {
            true
        }
    }
}

struct Solution;
