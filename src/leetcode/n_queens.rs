//! 51. N-Queens

use std::collections::HashSet;

struct Board {
    size: usize,
    under_attack: Vec<Vec<bool>>,
    results: Vec<Vec<String>>,
    queens: HashSet<(usize, usize)>
}

impl Board {

    fn new(size: usize) -> Self {
        Board {
            size,
            under_attack: vec![vec![false]],
            results: vec![],
            queens: HashSet::new()
        }
    }

    fn place(&mut self, row: usize, col: usize) {
        self.queens.insert((row, col));
        self.rebuild()
    }

    fn is_not_under_attack(&self, row: usize, col: usize) -> bool {
        !self.under_attack[row][col]
    }

    fn remove(&mut self, row: usize, col: usize) {
        self.queens.remove(&(row, col));
        self.rebuild();
    }

    fn save_result(&mut self) {
        let mut result = vec![vec!['.'; self.size]; self.size];
        for &(row, col) in &self.queens {
            result[row][col] = 'Q'
        }
        let result = result.into_iter().map(|row| row.iter().collect::<String>()).collect();
        self.results.push(result);
    }

    fn rebuild(&mut self) {
        self.under_attack = vec![vec![false; self.size]; self.size];

        fn is_diagonal(q_row: usize, q_col: usize, row: usize, col: usize) -> bool {
            (q_row as i32 - row as i32).abs() == (q_col as i32 - col as i32).abs()
        }

        for &(q_row, q_col) in &self.queens {
            for row in 0..self.size {
                self.under_attack[row][q_col] = true;
                for col in 0..self.size {
                    self.under_attack[q_row][col] = true;

                    if is_diagonal(q_row, q_col, row, col) {
                        self.under_attack[row][col] = true;
                    }
                }
            }
        }
    }
}


impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut board = Board::new(n as usize);
        Self::backtrack(&mut board, 0, 0);
        board.results
    }

    fn backtrack(board: &mut Board, row: usize, queens: usize) {
        for col in 0..board.size {
            if board.is_not_under_attack(row, col) {
                board.place(row, col);

                if queens + 1 == board.size {
                    board.save_result()
                } else {
                    Self::backtrack(board, row + 1, queens + 1)
                }

                board.remove(row, col);
            }
        }
    }
}

struct Solution;