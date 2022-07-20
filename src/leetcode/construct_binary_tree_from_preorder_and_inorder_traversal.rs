//! 105. Construct Binary Tree from Preorder and Inorder Traversal

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder_map = inorder
            .iter()
            .enumerate()
            .map(|(idx, val)| (*val, idx))
            .collect::<HashMap<_, _>>();
        Self::build(
            &preorder,
            &mut 0,
            0,
            preorder.len() as i32 - 1,
            &inorder_map,
        )
    }

    fn build(
        preorder: &[i32],
        idx: &mut usize,
        left: i32,
        right: i32,
        inorder_map: &HashMap<i32, usize>,
    ) -> Node {
        if left > right {
            return None;
        }
        let node_val = preorder[*idx];
        *idx += 1;
        let inorder_idx = *inorder_map.get(&node_val).unwrap() as i32;

        Some(Rc::new(RefCell::new(TreeNode {
            val: node_val,
            left: Self::build(preorder, idx, left, inorder_idx - 1, inorder_map),
            right: Self::build(preorder, idx, inorder_idx + 1, right, inorder_map),
        })))
    }
}
struct Solution;
