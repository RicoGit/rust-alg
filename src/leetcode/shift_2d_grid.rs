//! 1260. Shift 2D Grid

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        Self::sol2(grid, k)
    }

    // solution 1: one by one O(m*n*k) space O(1)
    fn sol1(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let row_size = grid[0].len();
        let col_size = grid.len();
        let n: usize = col_size * row_size;
        let k: usize = k as usize % n;

        let gris = &mut grid;
        for _ in 1..=k {
            let mut prev = grid[col_size - 1][row_size - 1];
            let mut tmp = 0;
            for row_idx in 0..col_size {
                for col_idx in 0..(row_size - 1) {
                    tmp = grid[row_idx][col_idx];
                    grid[row_idx][col_idx] = prev;
                    prev = tmp;
                }
                // last in a row
                tmp = grid[row_idx][row_size - 1];
                grid[row_idx][row_size - 1] = prev;
                prev = tmp;
            }
        }

        grid
    }

    // solution 2: copy to vector O(m*n) space O(m*n) [faster]
    fn sol2(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let n: usize = grid.len() * grid[0].len();
        let k: usize = k as usize % n;
        let mut left = vec![0; n - k];
        let mut right = vec![0; k];

        // copy to vec
        for row_idx in 0..grid.len() {
            for col_idx in 0..grid[row_idx].len() {
                let idx = row_idx * grid[row_idx].len() + col_idx;
                if idx < n - k {
                    left[idx] = grid[row_idx][col_idx];
                } else {
                    right[idx - (n - k)] = grid[row_idx][col_idx];
                }
            }
        }

        // println!("{:?}{:?}", left, right);
        right.extend(left.into_iter()); // swap

        // copy from vec
        for row_idx in 0..grid.len() {
            for col_idx in 0..grid[row_idx].len() {
                grid[row_idx][col_idx] = right[row_idx * grid[row_idx].len() + col_idx];
            }
        }

        grid
    }
}
struct Solution;
