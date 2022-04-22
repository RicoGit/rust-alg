//! 173. Binary Search Tree Iterator

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
use std::ops::DerefMut;
use std::ptr::{null, null_mut};
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

struct BSTIterator {
    stack: Vec<Node>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut iterator = BSTIterator { stack: vec![] };
        iterator.push_left(root);
        iterator
    }

    fn next(&mut self) -> i32 {
        let node_cell = self.stack.pop().unwrap().unwrap();
        let node = node_cell.borrow();

        if node.right.is_some() {
            self.push_left(node.right.clone())
        }

        node.val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

    fn push_left(&mut self, node: Node) {
        let mut cursor = node;
        while let Some(cursor_node) = cursor {
            cursor = cursor_node.borrow_mut().left.take();
            self.stack.push(Some(cursor_node));
        }
    }
}

struct Solution;
