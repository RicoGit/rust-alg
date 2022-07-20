//! 78. Subsets

//! 78. Subsets

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]];
        // 1 2 3
        // [[]] [[],[1]] [[],[1],[2],[1,2], [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
        for num in nums {
            for idx in 0..result.len() {
                let mut ans = result[idx].clone();
                ans.push(num);
                result.push(ans);
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
        let res = Solution::subsets(vec![1, 2, 3]);
        assert_eq!(
            res,
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ]
        )
    }
}
