//! 77. Combinations

impl Solution {

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        println!("k={},n={}", k, n);

        if k == 0 {
            return vec![vec![]];
        }

        let mut result: Vec<Vec<i32>> = vec![];

        for digit in k..=n {
            for mut solution in Self::combine(digit - 1, k - 1) {
                solution.push(digit);
                result.push(solution.clone());
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
        let res = Solution::combine(4, 2);
        assert_eq!(
            res,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![1, 4],
                vec![2, 4],
                vec![3, 4]
            ]
        )
    }
}
