//! 66. Plus One

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for idx in (0..digits.len()).rev() {
            if digits[idx] == 9 {
                digits[idx] = 0;
            } else {
                digits[idx] += 1;
                break;
            }
        }
        if let Some(0) = digits.first() {
            let mut result = vec![1];
            result.extend(digits.iter());
            result
        } else {
            digits
        }
    }
}

struct Solution;
