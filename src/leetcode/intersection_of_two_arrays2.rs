//! 350. Intersection of Two Arrays II

use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut map = HashMap::new();

        for num in nums1 {
            match map.insert(num, 1) {
                None => (),
                Some(val) => {
                    map.insert(num, val+1);
                },
            }
        }

        for num in nums2 {
            match map.remove(&num) {
                None => (),
                Some(freq) => {
                    if freq > 1 {
                        map.insert(num.clone(), freq-1);
                    }
                    result.push(num)
                }
            }
        }

        result
    }
}

struct Solution;
