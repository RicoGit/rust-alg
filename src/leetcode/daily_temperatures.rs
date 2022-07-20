//! 739. Daily Temperatures

impl Solution {
    pub fn daily_temperatures(temp: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temp.len()];
        let mut stack = vec![];

        for idx in 0..(temp.len() - 1) {
            let cur = temp[idx];
            let next = temp[idx + 1];

            while let Some(&(prev_num, prev_idx)) = stack.last() {
                if prev_num < next {
                    stack.pop();
                    result[prev_idx] = (idx - prev_idx + 1) as i32;
                } else {
                    break;
                }
            }

            if cur < next {
                result[idx] = 1;
            } else {
                stack.push((cur, idx));
            }
        }

        result
    }
}

struct Solution;
