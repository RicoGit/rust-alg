//! 344. Reverse String

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        for start_idx in 0..(s.len() / 2) {
            let tmp = s[start_idx];
            let end_idx = s.len() - 1 - start_idx;
            s[start_idx] = s[end_idx];
            s[end_idx] = tmp;
        }
    }
}
