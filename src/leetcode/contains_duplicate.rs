//! 217. Contains Duplicate

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let size = nums.len();
        nums.into_iter()
            .collect::<std::collections::HashSet<i32>>()
            .len()
            != size
    }
}

struct Solution;
