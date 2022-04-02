//! 189. Rotate Array

use std::cell::UnsafeCell;
use std::cmp::min;

impl Solution {

    // not efficient, but work well, consumes extra space
    pub fn rotate_simple(nums: &mut Vec<i32>, k: i32) {

        let len = nums.len();
        let point = len - (k as usize % len);

        let start = &nums[0..point];
        let end = &nums[point..nums.len()];

        *nums = [end, start].concat();
    }

    // efficient version with revert array, O(1) memory consumption
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize; // always non-negative
        let len = nums.len();
        let point = len - (k % len);

        // revert left part of array
        for idx in 0..(point / 2) {
            let tmp = nums[idx];
            nums[idx] = nums[point - idx - 1];
            nums[point - idx - 1] = tmp;
        }

        // revert right part of array  4 -6
        for idx in 0..(len - point) / 2 {
            let tmp = nums[idx + point];
            nums[idx + point] = nums[len - 1 - idx];
            nums[len - 1 - idx] = tmp;
        }

        nums.reverse();
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let mut input = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut input, 3);
        assert_eq!(input, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test2() {
        let mut input = vec![-1, -100, 3, 99];
        Solution::rotate(&mut input, 2);
        assert_eq!(input, vec![3, 99, -1, -100]);
    }
}
