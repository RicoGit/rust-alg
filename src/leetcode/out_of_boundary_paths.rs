//! 576. Out of Boundary Paths

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let mut dp = vec![vec![vec![-1 as i64; 52]; 52]; 52];
        Self::solve(
            start_row as usize + 1,
            start_column as usize + 1,
            max_move as usize,
            m as usize,
            n as usize,
            &mut dp,
        ) as i32
    }

    fn solve(
        row: usize,
        col: usize,
        moves: usize,
        m: usize,
        n: usize,
        dp: &mut Vec<Vec<Vec<i64>>>,
    ) -> i64 {
        if row < 1 || row > m || col < 1 || col > n {
            return 1;
        }
        if moves < 1 {
            return 0;
        }
        if dp[row][col][moves] != -1 {
            return dp[row][col][moves];
        }
        dp[row][col][moves] = (Self::solve(row - 1, col, moves - 1, m, n, dp)
            + Self::solve(row + 1, col, moves - 1, m, n, dp)
            + Self::solve(row, col - 1, moves - 1, m, n, dp)
            + Self::solve(row, col + 1, moves - 1, m, n, dp))
            % MOD;
        dp[row][col][moves]
    }
}

struct Solution;
