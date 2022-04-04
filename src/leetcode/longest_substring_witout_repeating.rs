//! 3. Longest Substring Without Repeating Characters

use std::ops::Index;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // string contains only one byte chars, it's safe to compare bytes
        let bytes: &[u8] = s.as_bytes();
        let mut seen: Vec<u8> = vec![];
        let mut max_seen: usize = 0;

        for idx in 0..bytes.len() {
            let current = bytes[idx];

            // todo optimize: use binary search for putting and retrieving from 'seen' vector
            match Solution::find_idx(&seen, &current) {
                Some(idx) => {
                    seen = Vec::from(&seen[idx+1..]);
                }
                None => ()
            }

            seen.push(current);

            let seen_len = seen.len();
            if max_seen < seen_len {
                max_seen = seen_len
            }
        }

        max_seen as i32
    }

    // linear search, binary search will be better here
    fn find_idx(arr: &[u8], el: &u8) -> Option<usize> {
        for (idx, elem) in arr.iter().enumerate() {
            if elem == el { return Some(idx) }
        }
        None
    }
}

struct Solution;
