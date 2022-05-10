//! 713. Subarray Product Less Than K

use std::collections::HashMap;

impl Solution {

    pub fn num_subarray_product_less_than_k(mut nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 { return 0 }

        let mut prod = 1;
        let mut result = 0;
        let mut left = 0;

        for right in 0..nums.len() {
            prod *= nums[right];
            while prod >= k {
                prod /= nums[left];
                left += 1;
            }
            result += right - left + 1;
        }

        result as i32
    }


    /// find all product less than k
    pub fn all_product_less_than_k_(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut result = 0;
        let current = nums[0];
        let mut stack = vec![];
        for num in nums.iter().cloned().rev() {
            if num < k {
                stack.push(num)
            }
        }

        while let Some(val) = stack.pop() {
            result += 1;
            println!("{:?}", val);
            let max_factor = k / val;

            match nums.binary_search(&max_factor) {
                Ok(idx) => {
                    for n in &nums[0..=idx] {
                        if *n > val {
                            stack.push(n*val);
                        }
                    }
                },
                Err(insert_idx) => {
                    for n in &nums[0..insert_idx] {
                        if *n > val {
                            stack.push(n*val);
                        }
                    }
                }
            }
        }

        result
    }
}

struct Solution;
