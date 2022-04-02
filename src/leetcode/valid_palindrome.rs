//! 125. Valid Palindrome

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .chars()
            .filter(|ch| ch.is_alphanumeric())
            .map(|ch| ch.to_ascii_lowercase())
            .collect();

        Solution::is_palindrome_word(&chars)
    }

    fn is_palindrome_word(chars: &[char]) -> bool {
        let mut start = 0;
        let mut end = chars.len() - 1;

        while start < end {
            if chars[start] == chars[end] {
                start += 1;
                end -= 1;
            } else {
                return false
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()), true)
    }
}
