//! 462. Minimum Moves to Equal Array Elements II

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        // let median = Self::simple_median(&mut nums);
        let median = Self::quick_select_median(&nums, nums.len()/2);
        let mut total = 0;
        for num in nums {
            total += (num - median).abs();
        }
        total
    }

    /// Sorting O(nlogn)
    fn simple_median(nums: &mut Vec<i32>) -> i32 {
        nums.sort();
        nums[nums.len()/2]
    }

    /// Quick select median O(logn)
    fn quick_select_median(nums: &[i32], idx: usize) -> i32 {

        fn gen(max: usize) -> usize {
            use rand::RngCore;
            let mut rng = rand::thread_rng();
            (rng.next_u64() as usize) % max
        }

        if nums.len() == 1 && idx == 0 {
            return nums[0]
        }

        let pivot_idx = gen(nums.len());
        let pivot = nums[pivot_idx];

        let mut left = vec![];
        let mut right = vec![];
        let mut pivots = vec![];
        for num in nums {
            if num < &pivot {
                left.push(*num);
            } else if num > &pivot {
                right.push(*num);
            } else {
                pivots.push(*num);
            }
        }

        if idx < left.len() {
            Self::quick_select_median(&left, idx)
        } else if idx < left.len() + pivots.len() {
            pivots[0]
        } else {
            Self::quick_select_median(&right, idx - left.len() - pivots.len())
        }
    }
}

struct Solution;
