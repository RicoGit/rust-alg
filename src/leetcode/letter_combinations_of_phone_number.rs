//! 17. Letter Combinations of a Phone Number

use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {

        let map = HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z'])
        ]);

        let mut result: Vec<String> = vec![];

        for digit in digits.chars() {
            let mut tmp = vec![];

            for letter in map.get(&digit).unwrap() {
                if result.len() == 0 {
                    tmp.push(letter.to_string())
                } else {
                    for res in result.iter() {
                        tmp.push(res.to_owned() + &letter.to_string())
                    }
                }
            }

            result = tmp
        }

        result
    }
}

struct Solution;
