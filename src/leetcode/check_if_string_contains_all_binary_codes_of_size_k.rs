//! 1461. Check If a String Contains All Binary Codes of Size K

use std::collections::HashSet;

impl Solution {
    // array (fast) space 0(2^k)
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        if s.len() < 2 {
            return false;
        }
        if k > s.len() {
            return false;
        }
        let target = 1 << k;
        let mut got = vec![false; target];

        for idx in 0..=s.len() - k {
            let cur = usize::from_str_radix(&s[idx..idx + k], 2).unwrap();
            got[cur] = true;
        }

        got.into_iter().filter(|n| *n).count() == target // 2^k
    }

    // hashset (slow), space O(2^k)
    pub fn has_all_codes_(s: String, k: i32) -> bool {
        let k = k as usize;
        if s.len() < 2 {
            return false;
        }
        if k > s.len() {
            return false;
        }

        let mut buf = HashSet::new();
        for idx in 0..=s.len() - k {
            let cur = (&s[idx..idx + k]).to_string();
            buf.insert(cur);
        }

        buf.len() == 1 << k // 2^k
    }
}

struct Solution;
