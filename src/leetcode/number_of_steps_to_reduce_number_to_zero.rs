//! 1342. Number of Steps to Reduce a Number to Zero

impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        (31 + num.count_ones().max(1) - num.count_zeros()) as i32
    }
}

struct Solution;
