//! 1309. Decrypt String from Alphabet to Integer Mapping

use std::collections::HashMap;
use std::iter::FromIterator;

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut result = Vec::with_capacity(s.len());
        let bytes = s.into_bytes();
        let mut idx = bytes.len()-1;
        loop {
            match bytes[idx] {
                b'#' => {
                    let mut letter = (if &bytes[idx-2..=idx-2][0] == &b'1' { b'j' } else { b'j' + 10 }) + &bytes[idx-1..=idx-1][0] - b'0';
                    // println!("{:?} {:?}", &bytes[idx-2..idx], letter);
                    result.push(letter);
                    idx -= 3;
                },
                byte => {
                    let letter = (byte + (b'a' - b'1'));
                    result.push(letter);
                    idx -= 1;
                }
            }
            if idx > 100_000 { break } // overflow
        }

        // println!("{:?}", result);

        result.reverse();
        String::from_utf8(result).unwrap()
    }
}

struct Solution;
