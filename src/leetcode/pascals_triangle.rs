//! 118. Pascal's Triangle

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = num_rows as usize;
        let mut res = vec![vec![]; n];
        res[0].push(1);

        for idx in 1..n {
            res[idx].push(1); // push first '1' in the row
            for col in 1..idx {
                let sum = res[idx - 1][col - 1] + res[idx - 1][col];
                res[idx].push(sum);
            }
            res[idx].push(1); // push last '1' in the row
        }
        res
    }
}

struct Solution;
