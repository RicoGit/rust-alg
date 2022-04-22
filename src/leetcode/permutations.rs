//! 46. Permutations

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![vec![]];
        }

        let mut result = vec![];

        for idx in 0..nums.len() {
            let mut nums_clone = nums.clone();
            nums_clone.remove(idx);

            for mut solution in Self::permute(nums_clone) {
                solution.push(nums[idx]);
                result.push(solution)
            }
        }

        result
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::permute(vec![1, 2, 3]);
        assert_eq!(
            res,
            vec![
                vec![3, 2, 1],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![1, 2, 3],
            ]
        )
    }
}
