//! 701. Insert into a Binary Search Tree

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

impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
        Self::insert(&root, val);
        root
    }

    fn insert(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
        if let Some(cell) = node {
            let mut node = cell.borrow_mut();
            if val < node.val {
                if node.left.is_none() {
                    node.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                } else {
                    Self::insert(&node.left, val);
                }
            } else {
                if node.right.is_none() {
                    node.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                } else {
                    Self::insert(&node.right, val);
                }
            }
        }
    }
}

struct Solution;
