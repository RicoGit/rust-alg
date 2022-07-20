//! 14. Longest Common Prefix

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs.first().unwrap().clone();
        for str in strs.into_iter() {
            prefix = Self::find_prefix(prefix, str);
        }
        prefix
    }

    #[inline]
    fn find_prefix(str1: String, str2: String) -> String {
        for idx in 0..(str1.len().min(str2.len())) {
            if &str1[idx..=idx] != &str2[idx..=idx] {
                return str1[0..idx].to_string();
            }
        }
        str1
    }
}

struct Solution;
