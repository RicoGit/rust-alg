//! 986. Interval List Intersections

impl Solution {
    pub fn interval_intersection(first: Vec<Vec<i32>>, second: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        let mut i = 0;
        let mut j = 0;
        while i < first.len() && j < second.len() {
            let (i_start, i_end) = (first[i][0], first[i][1]);
            let (j_start, j_end) = (second[j][0], second[j][1]);

            let start = i_start.max(j_start);
            let end = i_end.min(j_end);

            if start <= end {
                result.push(vec![start, end]);
            }

            if end == i_end {
                i += 1;
            } else {
                j += 1;
            }
        }

        result
    }
}

struct Solution;
