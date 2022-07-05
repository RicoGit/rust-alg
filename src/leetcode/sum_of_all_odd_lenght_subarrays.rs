//! 1588. Sum of All Odd Length Subarrays

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        for idx in (1..=arr.len()).step_by(2) {
            for sub_arr in arr.windows(idx) {
                result += sub_arr.iter().sum::<i32>();
            }
        }
        result
    }
}

struct Solution;