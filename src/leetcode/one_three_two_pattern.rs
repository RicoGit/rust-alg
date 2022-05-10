//! 456. 132 Pattern

impl Solution {

    /// fast O(n)
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut min = vec![i32::MAX; nums.len()];
        min[0] = nums[0];
        for idx in 1..nums.len() {
            min[idx] = min[idx - 1].min(nums[idx])
        }

        for j in (0..nums.len()).rev() {
            if nums[j] > min[j] {
                while !stack.is_empty() && *stack.last().unwrap() <= min[j] {
                    stack.pop();
                }
                if !stack.is_empty() && *stack.last().unwrap() < nums[j] {
                    return true;
                }
                stack.push(nums[j]);
            }
        }

        false
    }

    /// better brute force O(n^2)
    pub fn better_brute_force(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        let mut min_i = i32::MAX;
        for j in 0..(nums.len() - 1) {
            min_i = min_i.min(nums[j]);
            for k in (j + 1)..nums.len() {
                if nums[k] > min_i && nums[k] < nums[j] {
                    return true;
                }
            }
        }
        false
    }

    /// brute force O(n^3)
    pub fn brute_force(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        for i in 0..(nums.len() - 2) {
            for j in (i + 1)..(nums.len() - 1) {
                if nums[i] < nums[j] {
                    for k in (j + 1)..nums.len() {
                        if nums[k] > nums[i] && nums[k] < nums[j] {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

struct Solution;
