//! 566. Reshape the Matrix


impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let r = r as usize;
        let c = c as usize;

        if r * c != mat.len() * mat[0].len() {
            return mat
        }

        let mut row_idx = 0;
        let mut col_idx = 0;

        let mut res = vec![vec![0; c]; r];

        for row in mat {
            for el in row {
                res[row_idx][col_idx] = el;
                if col_idx == c-1 {
                    row_idx += 1;
                    col_idx = 0;
                } else {
                    col_idx += 1;
                }
            }
        }

        res
    }
}

struct Solution;
