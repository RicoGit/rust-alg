//! 383. Ransom Note

use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = HashMap::new();

        for ch in magazine.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }

        for ch in ransom_note.chars() {
            match map.remove(&ch) {
                None => return false,
                Some(val) => {
                    if val == 0 {
                        return false;
                    }
                    map.insert(ch, val - 1);
                }
            }
        }

        true
    }
}

struct Solution;
