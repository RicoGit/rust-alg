//! 300. Longest Increasing Subsequence

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut buf = vec![(1, 1); nums.len()];
        for end in 1..nums.len() {
            for start in 0..end {
                if nums[start] < nums[end] {
                    if buf[start].0 + 1 == buf[end].0 {
                        buf[end].1 += buf[start].1;
                    }
                    if buf[start].0 + 1 > buf[end].0 {
                        buf[end].0 = buf[start].0 + 1;
                        buf[end].1 = buf[start].1;
                    }
                }
            }
        }
        // println!("{:?}", buf);
        let mut longest = 0;
        let mut result = 0;
        for (dist, count) in buf {
            if longest == dist {
                result += count
            }
            if longest < dist {
                longest = dist;
                result = count;
            }
        }
        result
    }
}

struct Solution;
