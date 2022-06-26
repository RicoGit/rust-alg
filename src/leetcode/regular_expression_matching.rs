//! 10. Regular Expression Matching

impl Solution {
    // Bottom-Up always easy to write
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        let str: Vec<char> = s.chars().collect();
        let pat: Vec<char> = p.chars().collect();
        dp[s.len()][p.len()] = true;

        for s_idx in (0..=s.len()).rev() {
            for p_idx in (0..p.len()).rev() {
                let first_match = s_idx < s.len() && [str[s_idx], '.'].contains(&pat[p_idx]);

                if p_idx + 1 < p.len() && pat[p_idx+1] == '*' {
                    dp[s_idx][p_idx] = dp[s_idx][p_idx + 2] || first_match && dp[s_idx + 1][p_idx]
                } else {
                    dp[s_idx][p_idx] = first_match && dp[s_idx + 1][p_idx + 1]
                }
            }
        }
        dp[0][0]
    }
}

struct Solution;