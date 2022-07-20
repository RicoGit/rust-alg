//! 1143. Longest Common Subsequence

impl Solution {
    // dynamic programming
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut source = text1.chars().collect::<Vec<_>>();
        let mut target = text2.chars().collect::<Vec<_>>();

        let mut dp = vec![vec![0; target.len() + 1]; source.len() + 1];
        let mut result = 0;
        for s_idx in 1..=source.len() {
            for t_idx in 1..=target.len() {
                if source[s_idx - 1] == target[t_idx - 1] {
                    dp[s_idx][t_idx] = dp[s_idx - 1][t_idx - 1] + 1;
                    result = result.max(dp[s_idx][t_idx])
                } else {
                    dp[s_idx][t_idx] = dp[s_idx - 1][t_idx].max(dp[s_idx][t_idx - 1]);
                }
            }
        }
        // println!("{:?}", dp);
        result as i32
    }

    // wrong
    pub fn longest_common_subsequence_wrong(text1: String, text2: String) -> i32 {
        fn helper(source: &[char], target: &[char], count: usize, result: &mut usize) {
            if target.is_empty() || source.is_empty() {
                return;
            }

            for (idx, &ch) in source.iter().enumerate() {
                if ch == target[0] {
                    *result = usize::max(*result, count + 1);
                    helper(&source[idx..], &target[1..], count + 1, result)
                }
            }
        }

        let mut result = 0;
        let mut source = text1.chars().collect::<Vec<_>>();
        let mut target = text2.chars().collect::<Vec<_>>();
        helper(&source, &target, 0, &mut result);
        result as i32
    }
}

struct Solution;
