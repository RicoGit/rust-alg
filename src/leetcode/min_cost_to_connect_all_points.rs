//! 1584. Min Cost to Connect All Points

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

impl Solution {
    pub fn min_cost_connect_points(mut points: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![false; points.len()];
        let mut visited_count = 0;
        let mut result = 0;

        let mut distances = BinaryHeap::new();
        let mut current_idx = 0;

        while visited_count < points.len() - 1 {
            let node = &points[current_idx];

            // add distances for current node
            for idx in 0..points.len() {
                let cur = &points[idx];
                let dist = Self::distance(&node, cur);
                if current_idx != idx && !visited[idx] {
                    distances.push(Reverse((dist, idx)));
                }
            }

            loop {
                let Reverse((min_dist, p_idx)) = distances.pop().unwrap();
                if !visited[p_idx] && p_idx != current_idx {
                    visited[current_idx] = true;
                    visited_count += 1;
                    current_idx = p_idx;
                    result += min_dist;
                    break;
                }
            }
        }

        result
    }

    fn distance(p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
        (p2[0] - p1[0]).abs() + (p2[1] - p1[1]).abs()
    }
}

struct Solution;
