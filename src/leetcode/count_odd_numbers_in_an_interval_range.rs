//! 1523. Count Odd Numbers in an Interval Range

impl Solution {
    // fast O(1)
    #[inline]
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let count = (high - low) + 1;
        if count % 2 != 0 && low % 2 != 0 {
            (count / 2) + 1
        } else {
            (count / 2)
        }
    }

    // slow O(n)
    pub fn count_odds_(low: i32, high: i32) -> i32 {
        (low..=high).filter(|n| n % 2 != 0).count() as i32
    }
}

struct Solution;
