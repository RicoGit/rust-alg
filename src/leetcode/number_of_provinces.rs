//! 547. Number of Provinces

use std::collections::HashSet;

impl Solution {
    // [1,1,0]
    // [1,1,0]
    // [0,0,1]
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let size = is_connected.len();
        let mut visited = vec![false; size];
        let mut counter = 0;

        for city in 0..size {
            if visited[city] {
                continue;
            };
            visited[city] = true;
            counter += 1;
            let mut stack = vec![city];
            while let Some(city1) = stack.pop() {
                for city2 in 0..size {
                    if is_connected[city1][city2] == 0 || visited[city2] {
                        continue;
                    };
                    visited[city2] = true;
                    stack.push(city2)
                }
            }
        }
        counter
    }
}

struct Solution;
