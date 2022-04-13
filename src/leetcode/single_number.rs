impl Solution {
    // acc   el
    // 0000 
    // 1000 1000
    // 1010 0010 *
    // 0000 1010
    // 1010 1010
    // 0010 1000
    // res = 0010
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, el| acc ^ el)
    }
}

struct Solution;
