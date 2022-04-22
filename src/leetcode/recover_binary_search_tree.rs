//! 99. Recover Binary Search Tree

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
use std::cell::{RefCell, RefMut};
use std::collections::hash_map::DefaultHasher;
use std::ops::DerefMut;
use std::ptr::{null, null_mut};
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

struct Ctx {
    first: *mut TreeNode,
    second: *mut TreeNode,
    prev: *mut TreeNode,
    prev_val: i32,
}

impl Ctx {
    fn new() -> Self {
        Ctx {
            first: null_mut(),
            second: null_mut(),
            prev: null_mut(),
            prev_val: -1,
        }
    }
    fn fix(mut self) {
        unsafe {
            std::mem::swap(&mut (*self.first).val, &mut (*self.second).val);
        }
    }
}

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut ctx = Ctx::new();
        Self::helper(&root, &mut ctx);
        ctx.fix();
    }
    fn helper(root: &Node, ctx: &mut Ctx) {
        match root {
            None => (),
            Some(node_cell) => {
                let mut node = node_cell.borrow_mut();
                Self::helper(&node.left, ctx);
                visit(&mut node, ctx);
                Self::helper(&node.right, ctx);
            }
        }

        fn visit(mut node: &mut RefMut<TreeNode>, ctx: &mut Ctx) {
            if ctx.prev_val > node.val {
                if ctx.first.is_null() {
                    ctx.first = ctx.prev;
                }
                ctx.second = node.deref_mut();
            }

            ctx.prev = node.deref_mut();
            ctx.prev_val = node.val;
        }
    }
}

struct Solution;
