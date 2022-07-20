//! 343. Integer Break

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }

        let mut num = n;
        let mut result = 1;
        while num > 4 {
            num -= 3;
            result *= 3;
        }

        result * num
    }
}

struct Solution;
