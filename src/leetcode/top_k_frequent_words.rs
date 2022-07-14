//! 692. Top K Frequent Words

use std::iter::FromIterator;

impl Solution {
    pub fn top_k_frequent(mut words: Vec<String>, k: i32) -> Vec<String> {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        for w in words {
            *map.entry(w).or_insert(-1) -= 1
        }

        let mut freq = map.into_iter().map(|t| (t.1, t.0)).collect::<Vec<_>>();
        freq.sort();

        freq.into_iter().take(k as usize).map(|t| t.1).collect::<Vec<_>>()

    }
}

struct Solution;