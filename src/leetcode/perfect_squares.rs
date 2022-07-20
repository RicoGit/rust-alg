//! 279. Perfect Squares

use std::collections::VecDeque;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let max_square = (n as f64).sqrt();
        let squares = (1..=(max_square as usize))
            .map(|n| n * n)
            .collect::<Vec<_>>();
        let mut result = usize::MAX;
        let mut step = 0;
        let mut queue = VecDeque::new();
        queue.push_back(n as usize);

        while !queue.is_empty() && step < result {
            step += 1;
            for _ in 0..queue.len() {
                let remain = queue.pop_front().unwrap();
                for sq in squares.iter() {
                    if *sq > remain {
                        break;
                    }
                    let new_remain = remain - sq;

                    if new_remain == 0 && result > step {
                        result = step;
                        continue;
                    }

                    queue.push_back(new_remain)
                }
            }
            // println!("{:?}, step {}, res {}", queue, step, result);
        }

        result as i32
    }
}

struct Solution;
