//! 90. Subsets II

use std::collections::HashSet;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result = HashSet::new();
        result.insert(vec![]);

        for num in nums {
            let mut new_subset = HashSet::new();
            for old_subset in &result {
                let mut cloned = old_subset.clone();
                cloned.push(num);
                new_subset.insert(cloned);
            }

            result.extend(new_subset);
        }

        result.into_iter().collect()
    }
}

struct Solution;
