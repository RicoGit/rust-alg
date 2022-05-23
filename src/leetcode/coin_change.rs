//! 322. Coin Change

impl Solution {

    // dynamic programming
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 { return 0}
        let amount = amount as usize;
        let mut dp = vec![None; amount + 1];
        dp[0] = Some(0);

        for target in 1..=amount {
            for coin in coins.iter() {
                let coin = *coin as usize;
                if coin <= target {
                    if let Some(ans) = dp[target-coin].map(|n| n+1) {
                        dp[target] = Some(dp[target].unwrap_or(i32::MAX).min(ans))
                    }
                }
            }
        }

        // println!("{:?}", dp);
        dp[amount].unwrap_or(-1) as i32
    }

    // brute force
    pub fn coin_change_(mut coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 { return 0}
        coins.sort();
        let mut result = vec![];
        let mut min = usize::MAX;
        for idx1 in (0..coins.len()).rev() {
            Self::helper(&coins, amount, vec![], &mut result, &mut min);
        }
        println!("{:?} {}", result, min);
        min as i32
    }

    fn helper(coins: &[i32], remain: i32, mut ans: Vec<i32>, result: &mut Vec<Vec<i32>>, min: &mut usize) {
        if ans.len() > *min {
            return
        }
        for idx in (0..coins.len()).rev() {
            let coin = coins[idx];
            let new_remain = remain - coin;

            if new_remain == 0 {
                let mut ans = ans.clone();
                ans.push(coins[idx]);
                if ans.len() < *min {
                    *min = ans.len()
                }
                result.push(ans);
                continue
            }

            if new_remain < 0 {
                continue
            } else {
                ans.push(coins[idx]);
                Self::helper(coins, new_remain, ans.clone(), result, min)
            }
        }
    }
}

struct Solution;
