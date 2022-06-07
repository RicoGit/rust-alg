//! 583. Delete Operation for Two Strings

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut w1 = word1.chars().collect::<Vec<_>>();
        let mut w2 = word2.chars().collect::<Vec<_>>();

        let mut dp = vec![vec![0; w2.len()+1]; w1.len()+1];
        // ? 0 1 2 3
        // s 1 2 3 4
        // e 2 1 2 3
        // a 3 2 1 2
        //   ? e a t
        for idx1 in 0..=w1.len() {
            for idx2 in 0..=w2.len() {
                if idx1 == 0 || idx2 == 0 {
                    dp[idx1][idx2] = idx1 + idx2;
                    continue
                }
                if w1[idx1-1] == w2[idx2-1] {
                    dp[idx1][idx2] = dp[idx1-1][idx2-1];
                } else {
                    dp[idx1][idx2] = dp[idx1-1][idx2].min(dp[idx1][idx2-1]) + 1;
                }
            }
        }

        // println!("{:?}", dp);
        dp[w1.len()][w2.len()] as i32
    }
}

struct Solution;
