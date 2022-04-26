//! 235. Lowest Common Ancestor of a Binary Search Tree

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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        Self::helper(&root, p_val, q_val)
    }

    fn helper(root: &Node, p: i32, q: i32) -> Node {
        if let Some(cell) = root {
            let node = cell.borrow();
            let val = node.val;

            if p < val && q < val {
                Self::helper(&node.left, p, q)
            } else if p > val && q > val {
                Self::helper(&node.right, p, q)
            } else {
                root.clone()
            }
        } else {
            None
        }
    }
}

struct Solution;
