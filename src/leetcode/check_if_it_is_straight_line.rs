//! 1232. Check If It Is a Straight Line (not so easy)

impl Solution {
    pub fn check_straight_line(coords: Vec<Vec<i32>>) -> bool {
        if coords.len() == 2 { return true }

        fn slope(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
            (y2-y1) as f64 / (x2-x1) as f64
        }

        let k = slope(coords[0][0], coords[0][1], coords[1][0], coords[1][1]);

        if !k.is_finite() {
            return coords.windows(2).all(|pair| pair[0][0] == pair[1][0])
        }

        for idx in 2..coords.len() {
            if slope(coords[idx-1][0], coords[idx-1][1], coords[idx][0], coords[idx][1]) != k {
                return false
            }
        }

        true
    }
}

struct Solution;