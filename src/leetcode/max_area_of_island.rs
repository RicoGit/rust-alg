//! 695. Max Area of Island

impl Solution {

    // version 2
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        fn visit(grid: &Vec<Vec<i32>>, row: usize, col: usize, visited: &mut Vec<Vec<bool>>) -> i32 {
            let cur = grid[row][col];
            if cur == 0 || visited[row][col] { return 0 }
            visited[row][col] = true;

            let mut area = 1;
            if row > 0 { area += visit(&grid, row-1, col, visited) }
            if row < grid.len()-1 { area += visit(&grid, row+1, col, visited) }
            if col > 0 { area += visit(&grid, row, col-1, visited) }
            if col < grid[0].len()-1 { area += visit(&grid, row, col+1, visited) }

            area
        }

        let mut area = 0;
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if !visited[row][col] {
                    area = area.max(visit(&grid, row, col, &mut visited))
                }
            }
        }
        area
    }

    // version 1
    pub fn max_area_of_island_old(grid: Vec<Vec<i32>>) -> i32 {
        fn area(grid: &mut [Vec<i32>], row: usize, col: usize) -> i32 {
            let current = grid[row][col];

            if current != 1 {
                return 0;
            }
            grid[row][col] = 2;

            let mut sum = 1;
            if row > 0 {
                sum += area(grid, row - 1, col);
            }

            if row < grid.len() - 1 {
                sum += area(grid, row + 1, col);
            }

            if col > 0 {
                sum += area(grid, row, col - 1);
            }

            if col < grid[row].len() - 1 {
                sum += area(grid, row, col + 1);
            }

            sum
        }

        let mut grid = grid;
        let mut max = 0;

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                let current = grid[row][col];
                if current == 1 {
                    let res = area(&mut grid, row, col);
                    max = max.max(res);
                }
            }
        }
        max
    }
}

struct Solution;
