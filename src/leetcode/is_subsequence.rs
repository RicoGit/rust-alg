//! 392. Is Subsequence

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() { return true }
        if s.len() > t.len() { return false }
        let source = s.into_bytes();
        let mut idx = 0;
        for trg in t.bytes() {
            if trg == source[idx] { idx += 1 }
            if  idx == source.len() { return true }
        }

        false
    }
}

struct Solution;