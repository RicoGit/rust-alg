//! 230. Kth Smallest Element in a BST

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
    /// recursive alg is slow, we can make early return
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut res = 0;
        Self::helper(&root, k, 0, &mut res);
        res
    }

    fn helper(root: &Node, k: i32, counter: i32, res: &mut i32) -> i32 {
        match root  {
            None => counter,
            Some(node_cell) => {
                let node = node_cell.borrow();

                let counter = Self::helper(&node.left, k, counter, res) + 1;
                // println!("{}-{}", node.val, counter);
                if *res == 0 && k == counter {
                    *res = node.val;
                    return counter
                }
                Self::helper(&node.right, k, counter, res)
            }
        }

    }

}

struct Solution;
