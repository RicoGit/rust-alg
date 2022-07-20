//! 13. Roman to Integer

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut prev = 0;

        fn add_or_subtract(period: u32, prev: i32) -> i32 {
            let k = 10i32.pow(period - 1);
            if prev == (k * 10) / 2 || prev == k * 10 {
                -k
            } else {
                k
            }
        }

        for digit in s.chars().rev() {
            match digit {
                'I' => {
                    result += add_or_subtract(1, prev);
                    prev = 1;
                }
                'V' => {
                    result += 5;
                    prev = 5;
                }
                'X' => {
                    result += add_or_subtract(2, prev);
                    prev = 10;
                }
                'L' => {
                    result += 50;
                    prev = 50;
                }
                'C' => {
                    result += add_or_subtract(3, prev);
                    prev = 100;
                }
                'D' => {
                    result += 500;
                    prev = 500;
                }
                'M' => {
                    result += 1000;
                    prev = 1000;
                }
                _ => panic!("unknown gigit {}", digit),
            }
        }
        result
    }
}

struct Solution;
