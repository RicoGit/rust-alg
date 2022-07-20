//! 318. Maximum Product of Word Lengths

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut visited = deadends
            .into_iter()
            .map(|string| string.parse::<usize>().unwrap())
            .collect::<HashSet<_>>();
        let target = target.parse::<usize>().unwrap();
        let mut moves = 0;
        let mut queue = VecDeque::new();
        queue.push_back(0000);
        let cases = [1000, 0100, 0010, 0001];
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let lock = queue.pop_front().unwrap();

                if lock == target {
                    return moves;
                } else if visited.insert(lock) {
                    for case in &cases {
                        queue.push_back(Self::next(lock, *case, true)); // +
                        queue.push_back(Self::next(lock, *case, false)); // -
                    }
                }
            }
            moves += 1;
        }
        -1
    }

    fn next(lock: usize, case: usize, add: bool) -> usize {
        let wheel = (lock / case) % 10;
        let res = match (wheel, add) {
            (9, true) => lock - 9 * case,
            (_, true) => lock + case,
            (0, false) => lock + 9 * case,
            (_, false) => lock - case,
        };
        // println!("{} {} {} = {}", lock, case, add, res);
        res
    }
}

struct Solution;
