//! 205. Isomorphic Strings

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map = HashMap::new();
        let target = t.into_bytes();

        for (idx, src) in s.bytes().enumerate() {
            if let Some(&trg) = map.get(&src) {
                if trg != target[idx] {
                    return false;
                }
            } else {
                map.insert(src, target[idx]);
            }
        }

        map.keys().len() == map.values().collect::<HashSet<_>>().len()
    }
}

struct Solution;
