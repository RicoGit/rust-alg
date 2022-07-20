//! Max Number of K-Sum Pairs

use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        let mut counter = 0;

        for idx in 0..nums.len() {
            let n1 = nums[idx];
            let n2 = k - n1;

            if let Some(times) = map.remove(&n2) {
                counter += 1;
                if times > 1 {
                    map.insert(n2, times - 1);
                }
            } else {
                *map.entry(n1).or_insert(0) += 1;
            }
        }

        counter
    }
}

struct Solution;
