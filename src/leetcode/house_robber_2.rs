//! 213. House Robber II

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            2 => i32::max(nums[0], nums[1]),
            _ => i32::max(
                Self::dp(&nums[..(nums.len()-1)]),
                Self::dp(&nums[1..]),
            )
        }
    }

    fn dp(nums: &[i32]) -> i32 {
        let mut bigger = nums[0];
        let mut prev = nums[1];

        for idx in 2..nums.len() {
            let sum = bigger + nums[idx];
            bigger = i32::max(bigger, prev);
            prev = sum;
        }

        i32::max(bigger, prev)
    }
}

struct Solution;
