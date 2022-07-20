//! 1091. Shortest Path in Binary Matrix

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
            return -1;
        }

        let mut stack = VecDeque::new();
        stack.push_back((n - 1, n - 1));
        grid[n - 1][n - 1] = 10;

        while let Some((row, col)) = stack.pop_front() {
            let cur = grid[row][col];

            if row > 0 {
                Self::process(&mut grid, row - 1, col, cur, &mut stack);
                if col > 0 {
                    Self::process(&mut grid, row - 1, col - 1, cur, &mut stack)
                }
                if col < n - 1 {
                    Self::process(&mut grid, row - 1, col + 1, cur, &mut stack)
                }
            }

            if row < n - 1 {
                Self::process(&mut grid, row + 1, col, cur, &mut stack);
                if col > 0 {
                    Self::process(&mut grid, row + 1, col - 1, cur, &mut stack)
                }
                if col < n - 1 {
                    Self::process(&mut grid, row + 1, col + 1, cur, &mut stack)
                }
            }

            if col > 0 {
                Self::process(&mut grid, row, col - 1, cur, &mut stack)
            }

            if col < n - 1 {
                Self::process(&mut grid, row, col + 1, cur, &mut stack)
            }
        }

        if grid[0][0] > 9 {
            grid[0][0] - 10 + 1
        } else {
            -1
        }
    }

    fn process(
        grid: &mut Vec<Vec<i32>>,
        row: usize,
        col: usize,
        prev: i32,
        stack: &mut VecDeque<(usize, usize)>,
    ) {
        let num = grid[row][col];
        if num == 1 {
            return;
        }
        if num == 0 {
            grid[row][col] = prev + 1;
            stack.push_back((row, col));
            return;
        }
        let new_val = prev + 1;
        if new_val < num {
            grid[row][col] = new_val;
            stack.push_back((row, col));
        }
    }
}

struct Solution;
