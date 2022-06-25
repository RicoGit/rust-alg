//! 665. Non-decreasing Array

impl Solution {

    // working version
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        if nums.len() < 3 { return true }
        let mut count = 0;
        for idx in 1..nums.len() {
            if nums[idx-1] > nums[idx] {
                count += 1;
                if idx > 1 && nums[idx - 2] > nums[idx] {
                    nums[idx] = nums[idx - 1];
                }
            }
        }
        count <= 1
    }

    // doesn't work
    pub fn check_possibility_(nums: Vec<i32>) -> bool {
        if nums.len() < 2 { return true }
        let mut has_mistake = false;
        for win in nums.windows(3) {
            let (n1, n2, n3) = (win[0], win[1], win[2]);
            if n1 > n2 && n2 > n3 {
                return false
            }
            if n1 > n2 || n2 > n3 {
                if has_mistake {
                    return false
                } else {
                    has_mistake = true;
                }
            }
        }
        true
    }

}





struct Solution;