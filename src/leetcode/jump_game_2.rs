//! 45. Jump Game II

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut buf = vec![i32::MAX; n];
        buf[n - 1] = 0;
        for idx in (0..nums.len() - 1).rev() {
            let max_jump = (n - 1 - idx).min(nums[idx] as usize);
            for jump in 1..=max_jump {
                let jump_idx = idx + jump;
                if buf[jump_idx] != i32::MAX && buf[idx] > buf[jump_idx] + 1 {
                    buf[idx] = buf[jump_idx] + 1;
                }
            }
        }
        buf[0]
    }
}

struct Solution;
