//! 52. N-Queens II

use std::collections::HashSet;

#[derive(Debug)]
struct Board {
    board: Vec<Vec<bool>>,
    queens: HashSet<(usize, usize)>
}

impl Board {
    fn new(n: usize) -> Self {
        Board {
            board: vec![vec![false; n]; n],
            queens: HashSet::new()
        }
    }

    fn rebuild(&mut self) {
        fn is_diagonal(row1: usize, col1: usize, row2: usize, col2: usize) -> bool {
            (row1 as i32 - row2 as i32).abs() == (col1 as i32 - col2 as i32).abs()
        }

        let n = self.board.len();
        self.board = vec![vec![false; n]; n];

        for (row, col) in self.queens.iter() {
            for row_idx in 0..n {
                self.board[row_idx][*col] = true;
                for col_idx in 0..n {
                    if row_idx == *row {
                        self.board[row_idx][col_idx] = true;
                    }

                    if is_diagonal(*row, *col, row_idx, col_idx) {
                        self.board[row_idx][col_idx] = true
                    }
                }
            }
        }
    }

    fn place(&mut self, row: usize, col: usize) {
        self.queens.insert((row, col));
        self.rebuild()
    }

    fn is_not_under_attack(&self, row: usize, col: usize) -> bool {
        !self.board[row][col]
    }


    fn remove(&mut self, row: usize, col: usize) {
        if self.queens.remove(&(row, col)) {
            self.rebuild()
        }
    }
}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut board = Board::new(n);
        Self::backtrack(0, 0, n, &mut board) as i32
    }

    fn backtrack(row: usize, mut count: usize, n: usize, board: &mut Board) -> usize {
        for col in 0..n {
            if board.is_not_under_attack(row, col) {
                board.place(row, col);

                if row + 1 == n {
                    count += 1
                } else {
                    count = Self::backtrack(row + 1, count, n, board);
                }

                board.remove(row, col);
            }
        }
        count
    }
}

struct Solution;
