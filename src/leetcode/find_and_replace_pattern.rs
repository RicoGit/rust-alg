//! 890. Find and Replace Pattern

struct Solution;

use std::collections::{HashMap, HashSet};
use std::ops::Index;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {

        let mut result = vec![];

        for word in words {
            if Self::is_pattern(&word, &pattern) {
                result.push(word);
            }
        }

        result
    }

    #[inline]
    fn is_pattern(word: &str, pattern: &str) -> bool {
        let mut map = HashMap::new();

        if word.len() != pattern.len() { return false }

        for idx in 0..word.len() {
            let w_ch = &word[idx..=idx];
            let p_ch = &pattern[idx..=idx];

            if *map.entry(p_ch).or_insert(w_ch) != w_ch {
                return false
            }

        }

        if map.values().collect::<HashSet<&&str>>().len() != map.len() {
            return false
        };
        println!("{:?}", map);

        true
    }
}