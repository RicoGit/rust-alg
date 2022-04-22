//! 190. Reverse Bits

impl Solution {
    // solution 1: convert to string and reverse string
    // solution 2: shift to left and get one bit, then push this bit to result with shifting right
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut result = 0;
        for _ in 0..32 {
            result <<= 1;
            result |= x & 1;
            x >>= 1;
        }
        result
    }
}

struct Solution;
