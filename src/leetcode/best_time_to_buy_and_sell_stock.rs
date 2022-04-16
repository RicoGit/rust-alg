//! 121. Best Time to Buy and Sell Stock

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut low = i32::MAX;
        let mut profit = 0;

        for price in prices {
            low = low.min(price);
            profit = profit.max(price - low);
        }

        profit
    }
}

struct Solution;
