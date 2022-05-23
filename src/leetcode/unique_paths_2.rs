//! 63. Unique Paths II

impl Solution {
    pub fn unique_paths_with_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 || grid[0][0] == 1 { return 0 }
        let rows = grid.len();
        let cols = grid[0].len();
        let mut buf = vec![vec![0; cols]; rows];
        buf[0][0] = 1;

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == 1 { continue }
                if row > 0 {
                    buf[row][col] += buf[row-1][col];
                }
                if col > 0 {
                    buf[row][col] += buf[row][col-1];
                }
            }
        }

        println!("{:?}", buf);
        buf[rows-1][cols-1]
    }
}

struct Solution;
