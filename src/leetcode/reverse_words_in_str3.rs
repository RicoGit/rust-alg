//! 557. Reverse Words in a String III

use std::borrow::BorrowMut;
use std::ops::{Add, IndexMut};

impl Solution {
    // solution1: with std rev() function (short)
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .map(|word| word.chars().rev().collect())
            .collect::<Vec<String>>()
            .join(" ")
    }

    // solution2: without std rev() function
    pub fn reverse_words2(s: String) -> String {
        let mut words: Vec<String> = s
            .split_ascii_whitespace()
            .map(|word| Solution::reverse_copy_str(word).add(" "))
            .collect();

        words.last_mut().map(|w| w.remove(w.len() - 1));
        words.into_iter().collect::<String>()
    }

    fn reverse_copy_str(str: &str) -> String {
        let mut chars: Vec<char> = str.to_string().chars().collect();

        for idx in 0..chars.len() / 2 {
            let tmp = chars[idx];
            let end_idx = chars.len() - idx - 1;
            chars[idx] = chars[end_idx];
            chars[end_idx] = tmp;
        }

        chars.into_iter().collect::<String>()
    }

    // revert string with O(1) memory consumption
    // fn reverse_str(str: &mut str) {
    //     unsafe {
    //         let bytes = str.as_bytes_mut();
    //
    //         for idx in 0..bytes.len() / 2 {
    //             let tmp = bytes[idx];
    //             let end_idx = bytes.len() - idx - 1;
    //             bytes[idx] = bytes[end_idx];
    //             bytes[end_idx] = tmp;
    //         }
    //     }
    // }
}

struct Solution;
