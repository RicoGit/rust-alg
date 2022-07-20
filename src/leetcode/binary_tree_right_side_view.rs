//! 199. Binary Tree Right Side View

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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn bfs(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>, lvl: usize) {
            if let Some(node_ref) = node {
                let node = node_ref.borrow();

                if lvl == result.len() {
                    result.push(node.val);
                }
                bfs(&node.right, result, lvl + 1);
                bfs(&node.left, result, lvl + 1);
            }
        }

        let mut result = vec![];
        bfs(&root, &mut result, 0);
        result
    }
}

struct Solution;
