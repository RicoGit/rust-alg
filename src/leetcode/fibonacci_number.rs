//! 509. Fibonacci Number

impl Solution {
    pub fn fib(n: i32) -> i32 {
        pub fn fib_rec(num1: i32, num2: i32, n: i32) -> i32 {
            if n == 0 {
                return num1;
            }
            fib_rec(num2, num1 + num2, n - 1)
        }

        match n {
            0 => 0,
            1 => 1,
            n => fib_rec(0, 1, n),
        }
    }
}

struct Solution;
