//! 326. Power of Three


const MAX: i32 = 1162261467; // max possible value for i32 is 3^19 = 1162261467

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && MAX % n == 0
    }

    pub fn is_power_of_three_iter(mut n: i32) -> bool {
        if n < 1 { return false }

        while n % 3 == 0 {
            n /= 3;
        }

        return n == 1
    }
}

struct Solution;