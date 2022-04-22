//! 542. 01 Matrix

use std::collections::LinkedList;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len1 = mat.len();
        let len2 = mat[0].len();

        let mut res: Vec<Vec<i32>> = vec![vec![i32::MAX; len1]; len2];
        let mut queue: LinkedList<(usize, usize)> = LinkedList::new();

        for r in 0..len1 {
            for c in 0..len2 {
                if mat[r][c] == 0 {
                    queue.push_back((r, c));
                    res[r][c] = 0
                }
            }
        }

        while let Some((r, c)) = queue.pop_back() {
            if c > 0 {
                // check left
                if res[r][c - 1] > res[r][c] + 1 {
                    res[r][c - 1] = res[r][c] + 1;
                    queue.push_back((r, c - 1))
                }
            }

            if c < len2 - 1 {
                // check right
                if res[r][c + 1] > res[r][c] + 1 {
                    res[r][c + 1] = res[r][c] + 1;
                    queue.push_back((r, c + 1))
                }
            }

            if r > 0 {
                // check top
                if res[r - 1][c] > res[r][c] + 1 {
                    res[r - 1][c] = res[r][c] + 1;
                    queue.push_back((r - 1, c))
                }
            }

            if r < len1 - 1 {
                // check bottom
                if res[r + 1][c] > res[r][c] + 1 {
                    res[r + 1][c] = res[r][c] + 1;
                    queue.push_back((r + 1, c))
                }
            }
        }

        res
    }
}

// doesn't work, matrix isn't a square
pub fn update_matrix_(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![vec![]];
    let len = mat.len();
    for r in 0..len {
        let mut sum = 0;
        for c in 0..len {
            let mut sum = 0;
            for idx in 0..len {
                sum = mat[r][idx] + mat[idx][c];
            }
            res[r][c] = sum;
        }
    }

    res
}

struct Solution;
