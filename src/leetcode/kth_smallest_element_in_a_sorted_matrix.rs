//! 378. Kth Smallest Element in a Sorted Matrix

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap: BinaryHeap<_> = matrix
            .iter()
            .flat_map(|row| row.iter().map(|&num| Reverse(num)))
            .collect();
        for _ in 1..k {
            heap.pop();
        }
        heap.pop().unwrap().0
    }
}