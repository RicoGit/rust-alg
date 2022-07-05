//! 128. Longest Consecutive Sequence

use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = nums.iter().collect::<HashSet<&i32>>();

        let mut result = 0;

        for num in &nums {
            if !set.contains(&(num-1)) {
                let mut cur = *num;
                let mut count = 1;
                while set.contains(&(cur+1)) {
                    cur += 1;
                    count += 1;
                }

                result = result.max(count)
            }
        }

        result
    }
}
struct Solution;