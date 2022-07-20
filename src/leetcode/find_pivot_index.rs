//! 724. Find Pivot Index

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut right = nums.clone();
        for idx in (0..nums.len() - 1).rev() {
            right[idx] += right[idx + 1];
        }
        if right.len() == 1 || right[1] == 0 {
            return 0;
        }

        let mut sum = 0;
        for idx in 0..nums.len() {
            sum += nums[idx];
            if sum == right[idx] {
                return idx as i32;
            }
        }

        -1
    }
}

struct Solution;
