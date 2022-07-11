//! 424. Longest Repeating Character Replacement (quite difficult)

impl Solution {

    pub fn character_replacement(s: String, k: i32) -> i32 {

        let mut freq = vec![0; 26];
        let bytes = s.into_bytes();
        let (mut start, mut end) = (0, 0);
        let mut most_freq = 0;
        let mut result = 0;

        while end < bytes.len() {
            let idx = (bytes[end] - b'A') as usize;
            freq[idx] += 1;
            most_freq = most_freq.max(freq[idx]);

            while end - start + 1 - most_freq > k as usize {
                let idx = (bytes[start] - b'A') as usize;
                freq[(bytes[start] - b'A') as usize] -= 1;
                start += 1;
            }

            result = result.max(end - start + 1);
            end += 1;
        }
        result as i32
    }

}

struct Solution;
