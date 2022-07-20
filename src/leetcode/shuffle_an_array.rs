//! 384. Shuffle an Array

use rand::RngCore;

struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution { nums }
    }

    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    /// Fisher-Yates Algorithm
    fn shuffle(&self) -> Vec<i32> {
        let mut arr = self.nums.clone();
        let n = arr.len();
        for idx in 0..n {
            arr.swap(idx, Self::gen(n));
        }
        arr
    }

    fn gen(n: usize) -> usize {
        let mut rng = rand::thread_rng();
        (rng.next_u64() as usize) % n
    }
}
