//! 153. Find Minimum in Rotated Sorted Array

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums[0] <= nums[nums.len() - 1] {
            return nums[0];
        }

        let pivot = Self::find_pivot(&nums, 0, nums.len() - 1);
        nums[pivot]
    }

    fn find_pivot(nums: &Vec<i32>, start: usize, end: usize) -> usize {
        if start == end {
            return start + 1;
        }

        let mid = (start + end) / 2;
        if nums[start] < nums[mid] {
            Self::find_pivot(nums, mid, end)
        } else {
            Self::find_pivot(nums, start, mid)
        }
    }
}

struct Solution;
