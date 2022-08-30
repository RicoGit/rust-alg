//! 48. Rotate Image

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        // revert diagonal
        for row in 0..n {
            for col in row..n {
                if row == col { continue }
                let tmp = matrix[row][col];
                matrix[row][col] = matrix[col][row];
                matrix[col][row] = tmp;
            }
        }

        // revert left to right
        for row in 0..n {
            for col in 0..n/2 {
                let tmp = matrix[row][col];
                matrix[row][col] = matrix[row][n-col-1];
                matrix[row][n-col-1] = tmp;
            }
        }
    }
}