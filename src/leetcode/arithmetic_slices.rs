//! 413. Arithmetic Slices

impl Solution {
    // fast
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }

        let mut result = 0;
        let mut last = 0;

        for triple in nums.windows(3) {
            if triple[2] - triple[1] == triple[1] - triple[0] {
                result = result + last + 1;
                last = last + 1;
            } else {
                last = 0;
            }
        }
        result
    }

    // slow
    pub fn number_of_arithmetic_slices_(nums: Vec<i32>) -> i32 {
        let mut counter = 0;

        for idx1 in 1..nums.len() {
            let mut start = idx1 - 1;
            let mut diff = nums[idx1] - nums[idx1 - 1];
            for idx2 in idx1..nums.len() {
                let new_diff = nums[idx2] - nums[idx2 - 1];
                if new_diff == diff {
                    if idx2 - start > 1 {
                        counter += 1;
                    }
                } else {
                    diff = new_diff;
                    start = idx2;
                }
            }
        }
        counter
    }
}

struct Solution;
