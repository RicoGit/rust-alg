//! 26. Remove Duplicates from Sorted Array

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut last = 1;
        let mut prev = nums[0];
        for idx in 1..nums.len() {
            let cur = nums[idx];
            if prev != cur {
                nums[last] = cur;
                last += 1;
            }
            prev = cur;
        }
        last as i32
    }
}

struct Solution;