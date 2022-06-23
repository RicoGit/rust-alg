//! 1642. Furthest Building You Can Reach
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
        let mut biggest_jumps = BinaryHeap::new();
        for idx in 1..heights.len() {
            if heights[idx] > heights[idx - 1] {
                biggest_jumps.push(Reverse(heights[idx] - heights[idx-1]));
                if biggest_jumps.len() > ladders as usize {
                    if let Some(min) = biggest_jumps.pop() {
                        bricks -= min.0;
                        if bricks < 0 {
                            return idx as i32 - 1;
                        }
                    }
                }
            }
        }
        heights.len() as i32 - 1
    }
}

struct Solution;
