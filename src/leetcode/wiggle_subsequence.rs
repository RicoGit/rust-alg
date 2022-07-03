//! 376. Wiggle Subsequence

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 { return nums.len() as _ };

        // length of subsequences
        let (mut up, mut down) = (1, 1);

        for idx in 1..nums.len() {
            let cur = nums[idx];
            let prev = nums[idx-1];
            if cur > prev {
                up = down + 1;
            }
            if cur < prev {
                down = up + 1
            }
        }
        up.max(down)
    }
}

struct Solution;