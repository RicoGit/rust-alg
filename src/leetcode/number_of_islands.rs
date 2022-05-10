//! 200. Number of Islands

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut result = 0;

        for row_idx in 0..grid.len() {
            for col_idx in 0..grid[row_idx].len() {
                let cur = grid[row_idx][col_idx];
                if cur == '1' {
                    Self::mark_island(&mut grid, row_idx, col_idx);
                    result += 1;
                }
            }
        }

        result
    }

    fn mark_island(grid: &mut Vec<Vec<char>>, row_idx: usize, col_idx: usize) {
        let current = grid[row_idx][col_idx];

        if current != '1' {
            return
        }

        grid[row_idx][col_idx] = 'x'; // mark as visited

        if row_idx > 0 {
            Self::mark_island(grid, row_idx-1, col_idx)
        }

        if row_idx < grid.len() - 1 {
            Self::mark_island(grid, row_idx+1, col_idx)
        }
        if col_idx > 0 {
            Self::mark_island(grid, row_idx, col_idx-1)
        }

        if col_idx < grid[row_idx].len() - 1 {
            Self::mark_island(grid, row_idx, col_idx+1)
        }

    }
}

struct Solution;
