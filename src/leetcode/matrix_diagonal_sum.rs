//! 1572. Matrix Diagonal Sum

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let len = mat.len();
        let mut result = 0;
        for row in 0..len {
            for col in 0..len {
                if row == col || row + col == len - 1 {
                    result += mat[row][col];
                }
            }
        }
        result
    }
}

struct Solution;
