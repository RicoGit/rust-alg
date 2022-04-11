//! 198. House Robber

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut buf = vec![];
        Self::helper(&nums, &mut buf);
        println!("{:?}", buf);
        buf.into_iter().max().unwrap()
    }

    fn helper(nums: &[i32], buf: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0
        }

        let cur = nums[0];




        0
    }
}

struct Solution;
