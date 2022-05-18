//! 55. Jump Game

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut buf = vec![false; n];
        buf[n-1]=true;
        for idx in (0..nums.len()-1).rev() {
            // we start from end mark each idx in buf as true if path to last idx exists
            let mut max_jump = nums[idx] as usize;
            max_jump = max_jump.min(n - 1 - idx);
            for jump in 1..=max_jump {
                if buf[idx+jump] {
                    // we proof that last idx accessible from this idx
                    buf[idx] = true;
                    break
                }
            }
        };
        buf[0]
    }
}


struct Solution;
