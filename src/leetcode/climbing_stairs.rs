//! 70. Climbing Stairs

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut buf = vec![-1; nums.len()];
        for idx in 0..nums.len() {
            Self::helper(&nums, idx, &mut buf);
        }
        buf.into_iter().max().unwrap()
    }

    fn helper(nums: &[i32], start: usize, buf: &mut Vec<i32>) -> i32 {
        if start >= nums.len() {
            return 0;
        }

        let mut max = 0;

        for new_idx in (start + 2)..nums.len() {
            let res = if buf[new_idx] == -1 {
                Self::helper(nums, new_idx, buf)
            } else {
                buf[new_idx]
            };
            max = res.max(max);
        }

        buf[start] = nums[start] + max;
        buf[start]
    }
}

struct Solution;
