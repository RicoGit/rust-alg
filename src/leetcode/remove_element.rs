//! 27. Remove Element

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut insert_idx = 0;
        for idx in 0..nums.len() {
            let cur = nums[idx];
            if val != cur {
                nums[insert_idx] = cur;
                insert_idx += 1;
            }
        }
        insert_idx as i32
    }
}

struct Solution;