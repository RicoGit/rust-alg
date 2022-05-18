//! 62. Unique Paths

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut buf = vec![vec![0; n as usize]; m as usize];

        for row in 0..(m as usize) {
            for col in 0..(n as usize) {
                if row == 0 && col == 0 {
                    buf[0][0] = 1;
                    continue
                }
                let top = if row > 0 { buf[row-1][col] } else { 0 };
                let left = if col > 0 { buf[row][col-1] } else { 0 };
                buf[row][col] = top + left;
            }
        }
        buf[m-1][n-1]
    }
}

struct Solution;
