//! 695. Max Area of Island

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut max = 0;
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                let current = grid[row][col];
                if current == 1 {
                    let res = Solution::area(&mut grid, row, col);
                    max = max.max(res);
                    println!("{}", max);
                }
            }
        }
        max
    }

    fn area(grid: &mut [Vec<i32>], row: usize, col: usize) -> i32 {
        let current = grid[row][col];

        if current != 1 {
            return 0;
        }
        grid[row][col] = 2;

        let mut sum = 1;
        if row > 0 {
            sum += Solution::area(grid, row - 1, col);
        }

        if row < grid.len() - 1 {
            sum += Solution::area(grid, row + 1, col);
        }

        if col > 0 {
            sum += Solution::area(grid, row, col - 1);
        }

        if col < grid[row].len() - 1 {
            sum += Solution::area(grid, row, col + 1);
        }

        sum
    }
}

struct Solution;
