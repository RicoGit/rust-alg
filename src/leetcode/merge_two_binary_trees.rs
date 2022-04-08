//! 617. Merge Two Binary Trees

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

use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {

    /// Good recursive solution
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {

        match (root1.is_none(), root2.is_none()) {
            (true, true) => return None,
            (true, false) => return root2,
            (false, true) => return root1,
            _ => (),
        }

        let first: Rc<RefCell<TreeNode>> = root1.unwrap();
        let second: Rc<RefCell<TreeNode>> = root2.unwrap();
        let mut result;
        result = TreeNode::new(first.deref().borrow().val + second.deref().borrow().val);
        result.left = Self::merge_trees(first.deref().borrow().left.clone(), second.deref().borrow().left.clone());
        result.right = Self::merge_trees(first.deref().borrow().right.clone(), second.deref().borrow().right.clone());

        Some(Rc::new(RefCell::new(result)))
    }

    ///
    /// Converts tree to arrays, merge and convert back to tree (bad decision)
    ///
    pub fn merge_trees_2(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {

        // let it = root1.as_ref().unwrap();
        let mut first: Vec<i32> = vec![root1.as_ref().unwrap().as_ref().borrow().val];
        Solution::array_rec(&root1.unwrap(), 1, &mut first);
        let mut second: Vec<i32> = vec![root2.as_ref().unwrap().as_ref().borrow().val];
        Solution::array_rec(&root2.unwrap(), 1, &mut second);

        println!("{:?}{:?}", first, second);

        let result = if first.len() > second.len() {
            Solution::merge(first, second)
        } else {
            Solution::merge(second, first)
        };

        Solution::from_array(&result, 0)
    }

    fn array_rec(root: &Node, depth: usize, arr: &mut Vec<i32>) {
        let mut left_is_defined = false;
        match root.deref().borrow().left.clone() {
            Some(child) => {
                arr.push(child.deref().borrow().val.clone());
                left_is_defined = true;
                // Solution::array_rec(child, depth + 1, arr)
            }
            None => arr.push(-1),
        }

        let mut right_is_defined = false;
        match root.as_ref().borrow().right.clone() {
            Some(child) => {
                // arr[depth*2];
                arr.push(child.deref().borrow().val.clone());
                right_is_defined = true;
                // Solution::array_rec(child, depth + 1, arr)
            }
            None => arr.push(-1),
        }

        if left_is_defined {
            Solution::array_rec(&root.deref().borrow().left.clone().unwrap(), depth + 1, arr)
        }
        if right_is_defined {
            Solution::array_rec(&root.as_ref().borrow().right.clone().unwrap(), depth + 1, arr)
        }
    }

    fn merge(bigger: Vec<i32>, smaller: Vec<i32>) -> Vec<i32> {
        let mut result = bigger;
        for idx in 0..smaller.len() {
            result[idx] += smaller[idx]
        }

        result
    }

    fn from_array(arr: &[i32], depth: usize) -> Option<Node> {
        if let Some(num) = arr.first() {

            if *num == -1 {
                return None
            }

            let mut node = TreeNode::new(*num);
            if depth < arr.len() - 1{
                node.left = Solution::from_array(&arr[(1 * depth)..], depth + 1);
            }
            if 2 * depth < arr.len() - 1 {
                node.right = Solution::from_array(&arr[(2 * depth)..], depth + 1);
            }
            Some(Rc::new(RefCell::new(node)))
        } else {
            None
        }
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let first = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let second = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let third = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let res = Solution::merge_trees(first, second);
        assert_eq!(res, third)
    }
}
