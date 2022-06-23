//! 9. Palindrome Number

impl Solution {

    // convert to string
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 { return false }
        let bytes = x.to_string().into_bytes();
        for idx in 0..bytes.len()/2 {
            if bytes[idx] != bytes[bytes.len()-1-idx] {
                return false
            }
        }
        true
    }

    // basic, quite fast
    pub fn is_palindrome_(mut x: i32) -> bool {
        if x < 0 { return false }
        let mut digits = vec![];
        while x > 0 {
            digits.push(x % 10);
            x = x / 10;
        }
        for idx in 0..(digits.len())/2 {
            if digits[idx] != digits[digits.len()-1-idx] {
                return false
            }
        }

        true
    }
}

struct Solution;