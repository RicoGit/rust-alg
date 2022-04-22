//! 191. Number of 1 Bits

impl Solution {
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut counter = 0;
        while n != 0 {
            if n & 1 == 1 {
                counter += 1;
            }
            n >>= 1;
        }
        counter
    }
}

struct Solution;
