//! 709. To Lower Case

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        let diff = b'a' - b'A';
        let bytes = s
            .bytes()
            .map(|byte| {
                if byte >= b'A' && byte <= b'Z' {
                    byte + diff
                } else {
                    byte
                }
            })
            .collect::<Vec<u8>>();
        String::from_utf8(bytes).unwrap()
    }
}

struct Solution;
