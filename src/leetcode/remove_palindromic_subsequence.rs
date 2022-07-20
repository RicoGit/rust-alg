//! 1332. Remove Palindromic Subsequences

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if Self::is_palindrome(s) {
            1
        } else {
            2
        }
    }

    fn is_palindrome(s: String) -> bool {
        let mut start = 0;
        let mut end = s.len() - 1;

        while start < end {
            if &s[start..start + 1] != &s[end..end + 1] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }
}

struct Solution;
