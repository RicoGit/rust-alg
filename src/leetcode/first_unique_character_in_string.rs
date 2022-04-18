//! 387. First Unique Character in a String

use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {

        let mut map = HashMap::new();


        for ch in s.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }

        for (idx, ch) in s.char_indices() {
            if *map.get(&ch).unwrap() == 1 {
               return idx as i32
            }
        }
        -1
    }
}

struct Solution;
