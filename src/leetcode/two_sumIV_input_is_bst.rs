//! 653. Two Sum IV - Input is a BST

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
use std::collections::HashSet;
use std::rc::Rc;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut halves = HashSet::new();
        Self::helper(&root, k, &mut halves)
    }

    fn helper(node: &Option<Rc<RefCell<TreeNode>>>, k: i32, halves: &mut HashSet<i32>) -> bool {
        // println!("{:?}", halves);
        if let Some(cell) = node {
            let node = cell.borrow();

            if halves.contains(&node.val) {
                return true;
            } else {
                halves.insert(k - node.val);
                Self::helper(&node.left, k, halves) || Self::helper(&node.right, k, halves)
            }
        } else {
            false
        }
    }
}

struct Solution;
