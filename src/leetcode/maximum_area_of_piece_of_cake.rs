//! 1465. Maximum Area of a Piece of Cake After Horizontal and Vertical Cuts

impl Solution {
    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        fn max_diff(arr: &[i32]) -> i64 {
            let mut max = i32::MIN;
            for pair in arr.windows(2) {
                max = max.max(pair[1] - pair[0]);
            }
            max as i64
        }

        horizontal_cuts.push(0);
        horizontal_cuts.push(h);
        horizontal_cuts.sort();

        vertical_cuts.push(0);
        vertical_cuts.push(w);
        vertical_cuts.sort();

        (max_diff(&horizontal_cuts) * max_diff(&vertical_cuts) % 1_000_000_007) as i32
    }
}

struct Solution;
