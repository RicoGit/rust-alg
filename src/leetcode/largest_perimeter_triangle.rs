//! 976. Largest Perimeter Triangle

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for idx in (2..nums.len()).rev() {
            if nums[idx] < (nums[idx - 1] + nums[idx - 2]) {
                return nums[idx] + nums[idx - 1] + nums[idx - 2];
            }
        }
        0
    }
}

struct Solution;