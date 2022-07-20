//! 354. Russian Doll Envelopes

use std::cmp::{Ordering, Reverse};

impl Solution {
    pub fn max_envelopes(mut envelops: Vec<Vec<i32>>) -> i32 {
        let mut env = envelops
            .into_iter()
            .map(|vec| (vec[0], Reverse(vec[1])))
            .collect::<Vec<_>>();
        env.sort_unstable();

        let mut dp = vec![];

        for &(_, Reverse(h)) in &env {
            if let Some(i) = dp.binary_search(&h).err() {
                if i < dp.len() {
                    dp[i] = h;
                } else {
                    dp.push(h)
                }
            }
        }
        dp.len() as i32
    }

    // brute force [Time Limit Exceeded]
    pub fn max_envelopes_(mut env: Vec<Vec<i32>>) -> i32 {
        env.sort_by_key(|vec| vec[0]);
        let n = env.len();
        let mut buf = vec![1; n];

        for idx in 0..n {
            let w = env[idx][0];
            let h = env[idx][1];

            for next_idx in idx..env.len() {
                let next_w = env[next_idx][0];
                let next_h = env[next_idx][1];

                if next_h < h {
                    continue; // there is no suitable values in sorted array
                }

                if next_w > w && next_h > h {
                    buf[next_idx] = buf[next_idx].max(buf[idx] + 1);
                }
            }
        }
        *buf.iter().max().unwrap()
    }
}

struct Solution;
