//! 58. Length of Last Word

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut started = false;
        let mut result = 0;
        let whitespace = ' ' as u8;
        for letter in s.bytes().rev() {
            if letter == whitespace {
                if started {
                    return result;
                }
            } else {
                started = true;
                result += 1;
            }
        }
        result as i32
    }
}
struct Solution;