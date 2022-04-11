//! 347. Top K Frequent Elements

use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

        let mut freq = HashMap::new();

        for num in nums {
            match freq.remove(&num) {
                None => freq.insert(num, 1usize),
                Some(prev) => freq.insert(num, prev + 1)
            };
        };

        let mut res = freq.into_iter().collect::<Vec<(i32, usize)>>();
        res.sort_by_key(|it| Reverse(it.1));

        res.into_iter().map(|it| it.0).take(k as usize).collect()
    }
}

struct Solution;
