//! 139. Word Break

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut buf: Vec<bool> = vec![false; s.len() + 1];
        buf[0] = true;

        for idx in 0..buf.len() {
            if !buf[idx] {
                continue;
            }

            for word in word_dict.iter() {
                if idx + word.len() < buf.len() {
                    let candidate = &s[idx..idx + word.len()];
                    if candidate == *word {
                        buf[idx + word.len()] = true;
                    }
                }
            }
        }

        buf[s.len()]
    }
}

struct Solution;
