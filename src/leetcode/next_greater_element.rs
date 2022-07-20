//! 496. Next Greater Element I (not so easy as it seems)

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut prev_stack = vec![-1];
        let mut map = HashMap::new();

        for idx in (0..nums2.len()).rev() {
            let cur = nums2[idx];
            while let Some(prev) = prev_stack.pop() {
                if prev == -1 {
                    map.insert(cur, -1);
                    prev_stack.push(prev);
                    prev_stack.push(cur);
                    break;
                } else if cur < prev {
                    map.insert(cur, prev);
                    prev_stack.push(prev);
                    prev_stack.push(cur);
                    break;
                }
            }
        }

        nums1
            .into_iter()
            .map(|num| *map.get(&num).unwrap_or(&-1))
            .collect::<Vec<_>>()
    }
}

struct Solution;
