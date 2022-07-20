//! 438. Find All Anagrams in a String

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

impl Solution {
    pub fn find_anagrams(mut s: String, mut p: String) -> Vec<i32> {
        let p_arr = p.chars().collect::<Vec<_>>();
        let p_size = p_arr.len();

        let s_arr = s.chars().collect::<Vec<_>>();
        let s_size = s_arr.len();

        // fill p string char's frequencies
        let mut p_map = HashMap::new();
        for idx in 0..p_size {
            *p_map.entry(p_arr[idx]).or_insert(0) += 1
        }

        let mut candidate = HashMap::new();
        let mut result = vec![];

        for end_idx in 0..s_size {
            let cur = s_arr[end_idx];

            // add new
            *candidate.entry(cur).or_insert(0) += 1;

            // remove old if needed
            if end_idx >= p_size {
                let start_idx = end_idx - (p_size);
                let prev = s_arr[start_idx];
                if let Some(val) = candidate.remove(&prev) {
                    if (val - 1) > 0 {
                        candidate.insert(prev, val - 1);
                    }
                }
            }

            // check
            if candidate == p_map {
                let start_idx = end_idx - (p_size - 1);
                result.push(start_idx as i32);
            }
        }

        result
    }

    // slow
    pub fn find_anagrams_slow(mut s: String, mut p: String) -> Vec<i32> {
        let p_size = p.len();
        let s_size = s.len();

        let p_set: HashSet<u8> = HashSet::from_iter(p.bytes());
        let mut p_arr = p.into_bytes();
        p_arr.sort();
        let mut s_arr = s.into_bytes();

        let mut result = vec![];

        let mut end_idx = 0;
        let mut counter = 0;
        while end_idx < s_size {
            if p_set.contains(&s_arr[end_idx]) {
                counter += 1;
            } else {
                counter = 0;
            }
            end_idx += 1;

            if counter == p_size {
                let start_idx = end_idx - p_size;
                let mut candidate = (&s_arr[start_idx..end_idx]).to_vec();
                candidate.sort();
                // println!("{:?}", candidate.len());
                if compare_sorted(&p_arr, &candidate) {
                    result.push(start_idx as i32)
                }
                counter -= 1;
            }
        }

        result
    }
}

fn compare_sorted(arr1: &[u8], arr2: &[u8]) -> bool {
    for idx in 0..arr1.len() {
        if arr1[idx] != arr2[idx] {
            return false;
        }
    }
    true
}

struct Solution;
