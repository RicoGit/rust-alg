//! 1822. Sign of the Product of an Array

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut negative_cnt = 0;
        for num in nums {
            if num == 0 {
                return 0;
            }
            if num.is_negative() {
                negative_cnt += 1;
            }
        }
        if negative_cnt % 2 == 1 {
            -1
        } else {
            1
        }
    }
}
struct Solution;
