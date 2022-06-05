//! 202. Happy Number

use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let squares =
            vec![0,1,4,9,16,25,36,49,64,81];

        let mut visited =
            vec![116,38,73,58,89,145,42,20,4,16,37].into_iter().collect::<HashSet<usize>>();

        let mut num = n as usize;
        while num != 1 {
            let mut sum = 0;
            while num != 0 {
                let digit = num % 10;
                num /= 10;
                sum += squares[digit];
            }
            num = sum;
            if !visited.insert(num) {
                return false
            }
        }

        true
    }
}
struct Solution;