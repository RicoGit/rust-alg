//! 680. Valid Palindrome II

struct Solution;

impl Solution {

    fn is_valid_palindrome(chars: &[char]) -> bool {
        let mut start = 0;
        let mut end = chars.len() - 1;
        while start < end {
            // println!("{} == {}, {}-{}", chars[start], chars[end], start, end);

            if chars[start] == chars[end] {
                start +=1;
                end -= 1;
            } else {
                return false
            }
        }
        true

    }

    pub fn valid_palindrome(s: String) -> bool {

        let chars: Vec<char> = s.chars().collect();

        let mut start = 0;
        let mut end = chars.len() - 1;
        while start < end {
            // println!("{} == {}, {}-{}", chars[start], chars[end], start, end);

            if chars[start] == chars[end] {
                start +=1;
                end -= 1;
            } else {
                return
                    Solution::is_valid_palindrome(&chars[start..=(end-1)]) ||
                        Solution::is_valid_palindrome(&chars[start+1..=end])
            }
        }

        true

    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::valid_palindrome2::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::valid_palindrome("aba".to_string()), true)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::valid_palindrome("abba".to_string()), true)
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::valid_palindrome("abca".to_string()), true)
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::valid_palindrome("xaba".to_string()), true)
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::valid_palindrome("abc".to_string()), false)
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::valid_palindrome("hbakab".to_string()), true)
    }

    #[test]
    fn test7() {
        assert_eq!(
            Solution::valid_palindrome("ebcbbececabbacecbbcbe".to_string()),
            true
        )
    }
}
