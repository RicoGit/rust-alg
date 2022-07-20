//! 1480. Running Sum of 1d Array

impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for idx in 1..nums.len() {
            nums[idx] = nums[idx - 1] + nums[idx];
        }
        nums
    }
}

struct Solution;
