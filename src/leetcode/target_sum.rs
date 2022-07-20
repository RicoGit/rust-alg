//! 494. Target Sum

impl Solution {
    // bruteforce
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        fn helper(nums: &[i32], remain: i32, target: i32) -> i32 {
            if nums.is_empty() {
                return if remain == target { 1 } else { 0 };
            }
            helper(&nums[1..], remain + nums[0], target)
                + helper(&nums[1..], remain - nums[0], target)
        }
        helper(&nums, 0, target)
    }
}

struct Solution;
