//! 53. Maximum Subarray


impl Solution {
    //    -2, 1, -3, 4, -1, 2, 1, -5, 4

    // local:-2, global:-2
    // local:1,  global:1
    // local:-2, global:1
    // local:4,  global:4
    // local:3,  global:4
    // local:5,  global:5
    // local:6,  global:6
    // local:1,  global:6

    // Kadane's Algorithm
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut global_max = nums[0];
        let mut local_max = nums[0];

        for idx in 1..nums.len() {
            // println!("local:{}, global:{}", local_max, global_max);
            local_max = i32::max(nums[idx], local_max + nums[idx]);
            global_max = i32::max(local_max, global_max);
        }
        global_max
    }
}

struct Solution;
