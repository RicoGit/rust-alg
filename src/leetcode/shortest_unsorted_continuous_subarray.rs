//! 581. Shortest Unsorted Continuous Subarray

impl Solution {
    // O(n)
    pub fn find_unsorted_subarray(mut nums: Vec<i32>) -> i32 {
        let mut end = 0;
        let mut max = i32::MIN;

        for idx in 0..nums.len() {
            max = max.max(nums[idx]);
            if nums[idx] < max {
                end = idx;
            }
        }

        let mut min = i32::MAX;
        let mut start = 0;

        for idx in (0..nums.len()).rev() {
            min = min.min(nums[idx]);
            if nums[idx] > min {
                start = idx;
            }
        }

        (match end.checked_sub(start) {
            None => 0,
            Some(0) => 0,
            Some(v) => v + 1,
        }) as i32
    }

    // n lon n
    pub fn sort_solution(mut nums: Vec<i32>) -> i32 {
        let mut clone = nums.clone();
        clone.sort();

        let mut start = nums.len() - 1;
        let mut end = 0;
        for idx in 0..nums.len() {
            if nums[idx] != clone[idx] {
                start = start.min(idx);
                end = end.max(idx);
            }
        }

        (match end.checked_sub(start) {
            None => 0,
            Some(0) => 0,
            Some(v) => v + 1,
        }) as i32
    }
}

struct Solution;
