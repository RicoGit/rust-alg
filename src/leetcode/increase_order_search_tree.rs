//! 897. Increasing Order Search Tree


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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vec = vec![];
        Self::reverce_order(&root, &mut vec);

        let mut prev = None;
        for val in vec {
            prev = Some(Rc::new(RefCell::new(TreeNode {
                val,
                right: prev,
                left: None
            })))
        }
        prev

    }

    fn reverce_order(root: &Node, res: &mut Vec<i32>) {
        match root {
            None => (),
            Some(node_cell) => {
                let mut node = node_cell.borrow_mut();
                Self::reverce_order(&node.right, res);
                res.push(node.val);
                Self::reverce_order(&node.left, res);
            }
        }

    }
}

struct Solution;
