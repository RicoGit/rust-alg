//! 404. Sum of Left Leaves

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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
            match node {
                Some(node_cell) => {
                    let node = node_cell.borrow();
                    if !node.left.is_some() && !node.right.is_some() && is_left {
                        node.val
                    } else {
                        traverse(&node.left, true) + traverse(&node.right, false)
                    }
                }
                None => 0,
            }
        }
        traverse(&root, false)
    }
}

struct Solution;
