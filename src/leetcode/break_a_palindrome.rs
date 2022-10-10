//! 1328. Break a Palindrome

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        if palindrome.len() < 2 {
            return "".to_string();
        }

        let mut half = if palindrome.len() % 2 == 1 {
            (palindrome.len() / 2) - 1
        } else {
            palindrome.len() / 2
        };

        let mut chars = palindrome.chars().collect::<Vec<char>>();

        for idx in 0..=half {
            match chars[idx] {
                'a' => {} // do nothing
                _ => {
                    chars[idx] = 'a';
                    return chars.iter().collect::<String>();
                }
            }
        }

        chars[palindrome.len() - 1] = 'b'; // change last chars to b is always optimal choice
        chars.iter().collect::<String>()
    }
}

struct Solution;
