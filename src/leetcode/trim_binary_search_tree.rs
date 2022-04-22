//! 669. Trim a Binary Search Tree

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
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn trim_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let res = Self::helper(&root, low, high);
        // println!("{:?}", res);
        res
    }

    fn helper(root: &Node, low: i32, high: i32) -> Node {
        match root {
            None => None,
            Some(node_cell) => {
                let node = node_cell.borrow();
                let val = node.val;

                if node.val < low {
                    return Self::helper(&node.right, low, high);
                }

                if node.val > high {
                    return Self::helper(&node.left, low, high);
                }

                let left = Self::helper(&node.left, low, high);
                let right = Self::helper(&node.right, low, high);

                Some(Rc::new(RefCell::new(TreeNode { left, right, val })))
            }
        }
    }
}

struct Solution;
