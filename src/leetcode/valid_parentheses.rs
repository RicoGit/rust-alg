//! 20. Valid Parentheses

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<u8> = vec![];
        let mut bytes = s.into_bytes();

        for idx in 0..bytes.len() {
            let cur = bytes[idx];
            match cur {
                b'(' | b'[' | b'{' => stack.push(cur),
                b')' => {
                    if stack.pop() != Some(b'(') {
                        return false;
                    }
                }
                b']' => {
                    if stack.pop() != Some(b'[') {
                        return false;
                    }
                }
                b'}' => {
                    if stack.pop() != Some(b'{') {
                        return false;
                    }
                }
                other => (),
            }
        }

        stack.is_empty()
    }
}

struct Solution;
