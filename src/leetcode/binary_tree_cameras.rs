//! 968. Binary Tree Cameras

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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
enum Color {
    Red, Black, White
}

impl Color {
    fn next(self) -> Self {
        match self {
            Color::Red => { Color::Black }
            Color::Black => { Color::White }
            Color::White => { Color:: Red }
        }

    }
}

impl Solution {


    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut counter = 0;
        // todo
        counter
    }

    // wrong solution
    pub fn min_camera_cover_wrong(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // prev == true if previous node was black, false - if was red
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, red: &mut usize, black: &mut usize, white: &mut usize, prev: Color) {

            if let Some(cell) = root {
                let node = cell.borrow();

                let color = prev.next();
                match color {
                    Color::Red => { *red += 1 },
                    Color::Black => { *black += 1 }
                    Color::White => { *white += 1 }
                }

                dfs(&node.left, black, white, red, color.clone());
                dfs(&node.right, black, white, red, color);
            }
        }

        // colors
        let mut red = 0;
        let mut black = 0;
        let mut white = 0;
        dfs(&root, &mut red, &mut black, &mut white, Color::Red);
        black.min(red).max(1) as i32
    }


}

struct Solution;
