//! 1672. Richest Customer Wealth

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .into_iter()
            .map(|user| user.iter().sum::<i32>())
            .max()
            .unwrap()
    }
}

struct Solution;
