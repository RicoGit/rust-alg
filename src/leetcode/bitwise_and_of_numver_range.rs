//! 201. Bitwise AND of Numbers Range

impl Solution {

    pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        let mut shift = 0;

        while left != right {
            left >>= 1;
            right >>= 1;
            shift += 1
        }
        left << shift
    }


    pub fn range_bitwise_and_slow_iter(left: i32, right: i32) -> i32 {
        let mut result = right;
        for num in left..right {
            result &= num;
        }
        result
    }

    pub fn range_bitwise_and_slow_fold(left: i32, right: i32) -> i32 {
        let result = (left..right) // right not included
            .into_iter()
            .fold(right, |acc, num| {
                acc & num
            });

        result
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use crate::leetcode::bitwise_and_of_numver_range::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4)
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::range_bitwise_and(1, i32::MAX), 0)
    }
}