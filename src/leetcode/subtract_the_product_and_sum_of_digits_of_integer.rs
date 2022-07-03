//! 1281. Subtract the Product and Sum of Digits of an Integer

impl Solution {
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        let mut prod = 1;
        while n > 0 {
            let digit = n % 10;
            prod *= digit;
            sum += digit;
            n /= 10;
        }

        prod - sum
    }
}

struct Solution;