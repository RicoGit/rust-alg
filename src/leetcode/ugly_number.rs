//! 263. Ugly Number

impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        while n > 1 {
            if n % 5 == 0 {
                n /= 5
            } else if n % 3 == 0 {
                n /= 3
            } else if n % 2 == 0 {
                n /= 2
            } else {
                return false;
            }
        }

        true
    }
}

struct Solution;
