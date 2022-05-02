//! 34. Find First and Last Position of Element in Sorted Array

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() { return vec![-1, -1] }
        match Self::bin_search(&nums, target, 0, nums.len()-1) {
            None => vec![-1, -1],
            Some(start_idx) => {
                let mut end_idx = start_idx;
                while end_idx < nums.len() {
                    if nums[end_idx] == target {
                        end_idx += 1;
                    } else {
                        break
                    }
                }

                vec![start_idx as i32, (end_idx - 1) as i32]
            }
        }
    }

    fn bin_search(nums: &Vec<i32>, target: i32, start: usize, end: usize) -> Option<usize> {
        if nums[start] == target {
            return Some(start)
        }
        if start == end {
            return None
        }
        let mid = start/2 + end/2;
        if target > nums[mid] {
            Self::bin_search(nums, target, mid+1, end)
        } else {
            Self::bin_search(nums, target, start, mid)
        }
    }
}

struct Solution;
