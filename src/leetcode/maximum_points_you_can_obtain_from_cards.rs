//! 1423. Maximum Points You Can Obtain from Cards

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let total = card_points.iter().sum();
        if card_points.len() <= k { return total };

        let win_size = card_points.len()-k;
        let mut min_sum = (&card_points[0..win_size]).iter().sum::<i32>();
        let mut prev_sum = min_sum;

        for win in card_points.windows(win_size+1) {
            let new_sum = prev_sum - win[0] + win[win_size];
            min_sum = min_sum.min(new_sum);
            prev_sum = new_sum;
        }
        total - min_sum
    }
}

struct Solution;