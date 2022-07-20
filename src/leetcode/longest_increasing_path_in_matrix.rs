//! 329. Longest Increasing Path in a Matrix

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut buf = vec![vec![-1; matrix[0].len()]; matrix.len()];
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                Self::traverse(&matrix, row, col, &mut max, &mut buf);
            }
        }
        max as i32
    }

    fn traverse(
        matrix: &Vec<Vec<i32>>,
        row: usize,
        col: usize,
        max: &mut usize,
        buf: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if buf[row][col] == -1 {
            buf[row][col] = 1
        };

        if row > 0 && matrix[row][col] < matrix[row - 1][col] {
            let next_size = if buf[row - 1][col] == -1 {
                Self::traverse(matrix, row - 1, col, max, buf)
            } else {
                buf[row - 1][col]
            };
            buf[row][col] = buf[row][col].max(next_size + 1)
        }
        if row < matrix.len() - 1 && matrix[row][col] < matrix[row + 1][col] {
            let next_size = if buf[row + 1][col] == -1 {
                Self::traverse(matrix, row + 1, col, max, buf)
            } else {
                buf[row + 1][col]
            };
            buf[row][col] = buf[row][col].max(next_size + 1)
        }
        if col > 0 && matrix[row][col] < matrix[row][col - 1] {
            let next_size = if buf[row][col - 1] == -1 {
                Self::traverse(matrix, row, col - 1, max, buf)
            } else {
                buf[row][col - 1]
            };
            buf[row][col] = buf[row][col].max(next_size + 1)
        }
        if col < matrix[0].len() - 1 && matrix[row][col] < matrix[row][col + 1] {
            let next_size = if buf[row][col + 1] == -1 {
                Self::traverse(matrix, row, col + 1, max, buf)
            } else {
                buf[row][col + 1]
            };
            buf[row][col] = buf[row][col].max(next_size + 1)
        }
        let answer = buf[row][col];
        *max = usize::max(*max, answer as usize);
        answer
    }
}

struct Solution;
