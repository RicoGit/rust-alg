//! 1048. Longest String Chain

use std::cmp::Reverse;

impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        words.sort_by_key(|w| Reverse(w.len()));
        let mut cache = vec![1; words.len()];

        let mut answer = 0;
        for idx in 0..words.len() {
            answer = answer.max(Self::chain_length(&words, &mut cache, idx));
        }
        // println!("{:?}", words);
        // println!("{:?}", cache);
        answer
    }

    fn chain_length(words: &[String], cache: &mut [i32], idx: usize) -> i32 {

        if cache[idx] != 1 || words[idx].len() == 1 {
            return cache[idx]
        }

        for next_idx in idx+1..words.len() {
            let diff = words[idx].len() - words[next_idx].len();
            if diff == 0 { continue }
            if diff > 1 { break } // early return

            if is_predecessor(words[next_idx].as_bytes(), words[idx].as_bytes()) {
                cache[idx] = cache[idx].max(Self::chain_length(words, cache, next_idx) + 1);
            }
        }

        cache[idx]
    }

}

fn is_predecessor(w1: &[u8], w2: &[u8]) -> bool {
    if w1.len() == w2.len() - 1 {
        let mut idx1 = 0;
        let mut idx2 = 0;
        let mut not_eq = false;

        while idx1 < w1.len() || idx2 < w1.len() {
            if w1[idx1] == w2[idx2] {
                idx1 += 1;
                idx2 += 1;
            } else {
                if not_eq { return false }
                not_eq = true;
                idx2 += 1;
            }
        }

        true
    } else {
        false
    }
}

struct Solution;