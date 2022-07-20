//! 130. Surrounded Regions

use std::collections::HashSet;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        for row in 0..board.len() {
            for col in 0..board[0].len() {
                if row == 0 || col == 0 || row == board.len() - 1 || col == board[0].len() - 1 {
                    Self::bfs(board, row, col);
                }
            }
        }

        for row in 0..board.len() {
            for col in 0..board[0].len() {
                if board[row][col] == 'O' {
                    board[row][col] = 'X'
                }
                if board[row][col] == 'M' {
                    board[row][col] = 'O'
                }
            }
        }
    }

    fn bfs(mat: &mut Vec<Vec<char>>, row: usize, col: usize) {
        let cur = mat[row][col];
        if cur != 'O' {
            return;
        }
        mat[row][col] = 'M';

        if row > 0 {
            Self::bfs(mat, row - 1, col);
        }
        if row < mat.len() - 1 {
            Self::bfs(mat, row + 1, col);
        }
        if col > 0 {
            Self::bfs(mat, row, col - 1);
        }
        if col < mat[0].len() - 1 {
            Self::bfs(mat, row, col + 1);
        }
    }
}

struct Solution;
