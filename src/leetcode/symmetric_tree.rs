//! 101. Symmetric Tree

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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let node = root.borrow();
            Self::check(&node.left, &node.right)
        } else {
            true
        }
    }

    fn check(left: &Node, right: &Node) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(left_cell), Some(right_cell)) => {
                let left = left_cell.borrow();
                let right = right_cell.borrow();
                left.val == right.val
                    && Self::check(&left.left, &right.right)
                    && Self::check(&left.right, &right.left)
            }
        }
    }
}

struct Solution;
