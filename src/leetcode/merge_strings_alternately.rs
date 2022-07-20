//! 1768. Merge Strings Alternately

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let min_len = word1.len().min(word2.len());
        let total_len = word1.len() + word2.len();
        let mut result = String::with_capacity(total_len);
        for idx in 0..min_len {
            result.push_str(&word1[idx..=idx]);
            result.push_str(&word2[idx..=idx]);
        }

        if result.len() < total_len {
            let (large, small) = if word1.len() < word2.len() {
                (word2, word1)
            } else {
                (word1, word2)
            };
            result.extend(large.chars().skip(small.len()))
        }

        result
    }
}

struct Solution;
