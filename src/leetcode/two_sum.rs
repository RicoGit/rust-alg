//! 1. Two Sum

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // build map
        let mut map = std::collections::HashMap::<i32, usize>::new();
        for (idx, num) in nums.iter().enumerate() {
            map.insert(*num, idx);
        }

        // check numbers
        for (idx1, num) in nums.iter().enumerate() {
            let rem = target - num;
            match map.get(&rem) {
                Some(idx2) => {
                    if idx1 != *idx2 {
                        return vec![idx1 as i32, *idx2 as i32];
                    }
                }
                None => (),
            }
        }

        vec![]
    }
}

struct Solution;
