//! 209. Minimum Size Subarray Sum

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        for size in 1..=nums.len() {
            // check arrays of size 1, 2, etc.
            let mut acc = 0;
            for idx in 0..nums.len() {
                if idx >= size {
                    acc -= nums[idx - size];
                }

                acc += nums[idx];

                if acc >= target {
                    return size as i32;
                }
            }
        }

        0
    }

    // slow n^3
    pub fn brute_frce_solution(target: i32, nums: Vec<i32>) -> i32 {
        for size in 1..=nums.len() {
            // check arrays of size 1, 2, etc.
            for idx in 0..=nums.len() - size {
                let mut current = 0;
                for num in &nums[idx..idx + size] {
                    current += num
                }

                // println!("{:?}", current);
                if current >= target {
                    return size as i32;
                }
            }
        }
        0
    }
}

struct Solution;
