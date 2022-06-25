//! 2169. Count Operations to Obtain Zero

impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        fn helper(num1: i32, num2: i32) -> i32 {
            if num1 - num2 == 0 { return 1 }
            if num1 <= num2 {
                1 + helper(num1, num2-num1)
            } else {
                1 + helper(num1 - num2, num2)
            }

        }

        if num1 == 0 || num2 == 0 { return 0 }
        helper(num1, num2)

    }
}

struct Solution;