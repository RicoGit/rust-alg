//! 1502. Can Make Arithmetic Progression From Sequence

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let diff = arr[1] - arr[0];
        for idx in 2..arr.len() {
            if arr[idx] - arr[idx - 1] != diff {
                return false;
            }
        }
        true
    }
}

struct Solution;
