//! 72. Edit Distance

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        let mut dp = vec![vec![-1; n + 1]; m + 1];
        Self::solve(&word1, &word2, m, n, &mut dp)
    }

    fn solve(w1: &str, w2: &str, m: usize, n: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if m == 0 || n == 0 {
            return (m + n) as i32; // one string is over, need to delete or insert for equalising
        }
        if dp[m][n] != -1 {
            return dp[m][n];
        }
        if &w1[m - 1..m] == &w2[n - 1..n] {
            dp[m][n] = Self::solve(w1, w2, m - 1, n - 1, dp);
            dp[m][n]
        } else {
            let insert = Self::solve(w1, w2, m, n - 1, dp);
            let remove = Self::solve(w1, w2, m - 1, n, dp);
            let replace = Self::solve(w1, w2, m - 1, n - 1, dp);
            dp[m][n] = 1 + insert.min(remove).min(replace);
            dp[m][n]
        }
    }
}

struct Solution;
