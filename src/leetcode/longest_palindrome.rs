//! 409. Longest Palindrome

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut buf = vec![0; (b'z' - b'A') as usize + 1];
        for byte in s.bytes() {
            let idx = (byte - b'A') as usize;
            buf[idx] += 1;
        }

        println!("{:?}", buf);

        let mut result = 0;
        for cnt in buf {
            if cnt % 2 == 0 { result += cnt; }
            if cnt % 2 == 1 {
                if result % 2 == 1 { result += cnt - 1 } else { result += cnt }
            }
        }

        result
    }
}


struct Solution;