//! 78. Subsets

impl Solution {
    // []
    // [] [1]
    // [] [1] [2] [1,2]
    // [] [1] [2] [1,2] [3] [1,3] [1,2,3]
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]];

        for num in nums {
            let mut new_subset = vec![];

            for mut old_subset in &mut result {
                let mut subset = old_subset.clone();
                subset.push(num);
                new_subset.push(subset)
            }

            result.extend(new_subset)
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
        let res = Solution::subsets(vec![1,2,3]);
        assert_eq!(
            res,
            vec![
                vec![], vec![1], vec![2], vec![1, 2], vec![3], vec![1, 3], vec![2, 3], vec![1, 2, 3],
            ]
        )
    }
}
