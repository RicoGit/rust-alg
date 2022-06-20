//! 820. Short Encoding of Words

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut reversed: Vec<String> = words.into_iter().map(|w| w.chars().rev().collect()).collect();
        reversed.sort();
        let mut result = reversed[0].len() + 1;
        for pair in reversed.windows(2) {
            let (first, second) = (&pair[0], &pair[1]);
            if second.starts_with(first) {
                result = result - first.len() + second.len();
            } else {
                result += second.len() + 1
            }
        }

        result as i32
    }
}

struct Solution;