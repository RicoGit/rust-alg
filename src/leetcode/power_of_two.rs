//! 231. Power of Two


impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & n - 1 == 0
    }
}


struct Solution;


#[cfg(test)]
mod test {
    use crate::leetcode::power_of_two::Solution;

    #[test]
    fn test1() {
       assert!(Solution::is_power_of_two(8));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_power_of_two(6));
    }
}
