//! 74. Search a 2D Matrix

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 {
            return false;
        }
        let len = matrix.len() * matrix[0].len();
        Self::search(&matrix, 0, len - 1, target)
    }

    fn search(mat: &[Vec<i32>], start: usize, end: usize, target: i32) -> bool {
        if start == end {
            if Self::get(mat, start) == target {
                return true;
            } else {
                return false;
            }
        }
        let mid = end / 2 + start / 2;

        if target > Self::get(mat, mid) {
            Self::search(mat, mid + 1, end, target)
        } else {
            Self::search(mat, start, mid, target)
        }
    }

    fn get(mat: &[Vec<i32>], idx: usize) -> i32 {
        let row_idx = idx / mat[0].len();
        let col_idx = idx % mat[0].len();
        mat[row_idx][col_idx]
    }
}

struct Solution;
