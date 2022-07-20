//! 394. Decode String

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = vec![];
        let mut result = String::from("");
        let mut repeat = 0;

        for c in s.chars() {
            match c {
                '0'..='9' => repeat = repeat * 10 + (c as u8 - b'0') as usize,
                '[' => {
                    stack.push((repeat, result.clone()));
                    repeat = 0;
                    result.clear()
                }
                ']' => {
                    if let Some((n, s)) = stack.pop() {
                        result = s + result.repeat(n).as_str()
                    }
                }
                c => result.push(c),
            }
        }

        result
    }
}

struct Solution;
