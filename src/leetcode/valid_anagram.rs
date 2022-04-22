//! 242. Valid Anagram

use std::collections::HashMap;

impl Solution {
    // sorting: S los S + T log T
    // hashMap: s + t
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = HashMap::new();
        for ch in s.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            let val = map.entry(ch).or_default();
            if *val == 0 {
                return false;
            };
            *val = *val - 1;
        }

        true
    }
}

struct Solution;
