//! Combinations pairs

impl Solution {

    /// Recursive solution
    pub fn combine(n: i32) -> Vec<(i32, i32)> {
        Self::combine_rec(n, n)
    }

    pub fn combine_rec(digit: i32, n: i32) -> Vec<(i32, i32)> {

        if digit == 0 {
            return vec![]
        }

        let mut res = Self::combine_rec(digit-1, n);

        println!("{:?},cur={}", res, digit);

        for second in digit+1..=n {
            res.push((digit, second));
        }
        res
    }

}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::combine(4);
        assert_eq!(
            res,
            vec![
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 3),
                (2, 4),
                (3, 4)
            ]
        )
    }
}
