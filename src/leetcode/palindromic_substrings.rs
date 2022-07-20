//! 647. Palindromic Substrings

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let bytes = s.into_bytes();
        let n = bytes.len();
        let mut is_palindrome = vec![vec![false; n]; n];
        let mut counter = 0;

        for end in 0..n {
            for start in 0..=end {
                let first = bytes[start];
                let last = bytes[end];
                if first == last && (end - start <= 2 || is_palindrome[start + 1][end - 1]) {
                    is_palindrome[start][end] = true;
                }
                if is_palindrome[start][end] {
                    counter += 1;
                }
            }
        }
        println!("{:?}", is_palindrome);
        counter
    }
}

struct Solution;
