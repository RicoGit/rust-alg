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

impl Solution {

    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

        #[derive(Clone, PartialEq, Eq)]
        enum Camera {  HasCamera, Covered, NeedCover }

        fn solve(root: &Option<Rc<RefCell<TreeNode>>>, counter: &mut i32) -> Camera {
            if root.is_none() { return Camera::Covered }

            let node = root.as_ref().unwrap().borrow();

            match (solve(&node.left, counter), solve(&node.right, counter)) {
                (Camera::NeedCover, _) | (_, Camera::NeedCover) => {
                    *counter += 1;
                    return Camera::HasCamera
                }
                (Camera::HasCamera, _) | (_, Camera::HasCamera) => {
                    return Camera::Covered
                }
                (Camera::Covered, _) | (_, Camera::Covered) => {
                    return Camera::NeedCover
                }
            }
        }

        let mut counter = 0;
        if solve(&root, &mut counter) == Camera::NeedCover {
            counter + 1
        } else {
            counter
        }
    }

    // wrong solution
    pub fn min_camera_cover_wrong(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

        #[derive(Clone)]
        enum Color { Red, Black, White }

        impl Color {
            fn next(self) -> Self {
                match self {
                    Color::Red => { Color::Black }
                    Color::Black => { Color::White }
                    Color::White => { Color:: Red }
                }

            }
        }

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
