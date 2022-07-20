//! 215. Kth Largest Element in an Array

use std::collections::BinaryHeap;

impl Solution {
    // with MinHeap (100%, 42%)
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut largest = BinaryHeap::with_capacity(k as usize);
        for num in nums {
            largest.push(-num);
            if largest.len() > k as usize {
                largest.pop();
            }
        }
        // println!("{:?}", largest);
        -largest.pop().unwrap()
    }

    // with sorting (40%, 40%)
    pub fn find_kth_largest_sort(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        nums[nums.len() - k as usize]
    }
}

struct Solution;
