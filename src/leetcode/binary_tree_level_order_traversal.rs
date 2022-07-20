//! 102. Binary Tree Level Order Traversal

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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let Some(node) = &root {
            let mut res = vec![vec![node.borrow().val]];
            Self::bfs(&root, &mut res, 1);
            return res;
        }
        vec![]
    }

    fn bfs(node: &Node, res: &mut Vec<Vec<i32>>, depth: usize) {
        match node {
            None => (),
            Some(node_cell) => {
                let node = node_cell.borrow();

                Self::put(&node.left, res, depth);
                Self::put(&node.right, res, depth);

                Self::bfs(&node.left, res, depth + 1);
                Self::bfs(&node.right, res, depth + 1);
            }
        }
    }

    fn put(node: &Node, res: &mut Vec<Vec<i32>>, depth: usize) {
        node.as_ref().map(|n| {
            if depth == res.len() {
                res.push(vec![]) // once for lvl
            }
            res[depth].push(n.borrow().val)
        });
    }
}

struct Solution;
