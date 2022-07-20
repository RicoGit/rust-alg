//! 135. Candy

impl Solution {
    // two passes
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut buf = vec![1; ratings.len()];

        for idx in 1..ratings.len() {
            if ratings[idx - 1] < ratings[idx] {
                buf[idx] = buf[idx - 1] + 1;
            }
        }

        let mut result = *buf.last().unwrap();
        for idx in (0..ratings.len() - 1).rev() {
            if ratings[idx] > ratings[idx + 1] {
                buf[idx] = buf[idx].max(buf[idx + 1] + 1);
            }
            result += buf[idx];
        }

        result
    }
}

struct Solution;
