//! 1302. Deepest Leaves Sum

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
use std::collections::HashMap;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = HashMap::new();
        Self::traverse(&root, 0, &mut result);
        // println!("{:?}", result);
        *result.iter().max().unwrap().1 as i32
    }

    fn traverse(root: &Node, depth: usize, result: &mut HashMap<usize, usize>) -> bool {
        if let Some(cell) = root {
            let node = cell.borrow();
            let left_is_empty = Self::traverse(&node.left, depth + 1, result);
            let right_is_empty = Self::traverse(&node.right, depth + 1, result);

            if left_is_empty && right_is_empty {
                // we're faced leaf
                *result.entry(depth).or_insert(0) += node.val as usize;
            }

            false
        } else {
            true
        }
    }
}

struct Solution;
