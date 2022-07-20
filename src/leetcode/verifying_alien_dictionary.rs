//! 953. Verifying an Alien Dictionary

use std::collections::HashMap;
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        fn compare(first: &str, second: &str, dict: &HashMap<u8, usize>) -> bool {
            let mut idx = 0;
            while &first[idx..=idx] == &second[idx..=idx] {
                idx += 1;
                if idx == first.len() {
                    return second.len() >= first.len();
                } else if idx == second.len() {
                    return false;
                }
            }
            dict.get(&first[idx..=idx].as_bytes()[0]).unwrap()
                < dict.get(&second[idx..=idx].as_bytes()[0]).unwrap()
        }

        let dict = order
            .bytes()
            .enumerate()
            .map(|(idx, byte)| (byte, idx))
            .collect::<HashMap<u8, usize>>();

        for pair in words.windows(2) {
            if !compare(&pair[0], &pair[1], &dict) {
                return false;
            }
        }
        true
    }
}

struct Solution;
