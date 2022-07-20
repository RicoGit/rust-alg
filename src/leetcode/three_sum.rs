//! 15. 3Sum

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

impl Solution {
    // fast 50ms
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        };
        let mut result = vec![];

        let mut nums = nums;
        nums.sort();

        let mut idx = 0;
        while idx < nums.len() - 2 {
            let target = -nums[idx]; // two-sum problem: x + y = -z
            let mut x = idx + 1;
            let mut y = nums.len() - 1;

            while x < y {
                let sum = nums[x] + nums[y];
                if sum < target {
                    x += 1;
                } else if sum > target {
                    y -= 1;
                } else {
                    result.push(vec![-target, nums[x], nums[y]]);

                    while x < y {
                        if nums[x + 1] == nums[x] {
                            x += 1;
                        } else if nums[y - 1] == nums[y] {
                            y -= 1;
                        } else {
                            x += 1;
                            y -= 1;
                            break;
                        }
                    }
                }
                while (idx < nums.len() - 2) && (nums[idx] == nums[idx + 1]) {
                    idx += 1;
                }
            }
            idx += 1;
        }

        result
    }

    // slow  500ms
    pub fn hash_map_three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        };
        let mut set =
            HashMap::<i32, usize>::from_iter(nums.iter().enumerate().map(|(idx, num)| (*num, idx)));
        let mut result = HashSet::new();

        for i in 0..nums.len() - 2 {
            for j in (i + 1)..nums.len() - 1 {
                let k_target = 0 - nums[i] - nums[j];
                if let Some(&k) = set.get(&k_target) {
                    if k > j {
                        let mut answer = vec![nums[i], nums[j], nums[k]];
                        answer.sort();
                        result.insert(answer);
                    }
                }
            }
        }

        result.into_iter().collect::<Vec<_>>()
    }
}

struct Solution;
