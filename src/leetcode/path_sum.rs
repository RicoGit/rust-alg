//! 112. Path Sum


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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false
        }
        Self::sum(&root, 0, target_sum)
    }

    fn sum(node: &Node, sum: i32, target: i32) -> bool {
        if let Some(node_cell) = node {
            let node = node_cell.borrow();
            let sum = sum + node.val;

            if sum == target && Self::is_leaf(&node) {
                true
            } else {
                Self::sum(&node.left, sum, target) || Self::sum(&node.right, sum, target)
            }
        } else {
            false
        }

    }

    fn is_leaf(node: &TreeNode) -> bool {
        node.left.is_none() && node.right.is_none()
    }
}

struct Solution;
