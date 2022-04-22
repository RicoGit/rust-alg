//! 283. Move Zeroes

impl Solution {
    // looks like a bubble sort O(n^2)
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut idx = 0;
        let mut first_zero_idx = nums.len() - 1;

        while idx < first_zero_idx {
            if nums[idx] == 0 {
                let mut current_idx = idx;

                while current_idx < first_zero_idx {
                    Solution::swap(nums, current_idx);
                    current_idx += 1;
                }

                first_zero_idx -= 1;
            }

            if nums[idx] != 0 {
                idx += 1;
            }
        }
    }

    fn swap(nums: &mut [i32], idx: usize) {
        let tmp = nums[idx];
        nums[idx] = nums[idx + 1];
        nums[idx + 1] = tmp
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0])
    }

    #[test]
    fn test2() {
        let mut nums = vec![0, 0, 1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0, 0])
    }
}
