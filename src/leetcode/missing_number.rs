//! 268. Missing Number

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let expected: usize = (0..=nums.len()).sum();
        let sum: i32 = nums.iter().sum();

        (expected as i32 - sum)
    }
}

struct Solution;
