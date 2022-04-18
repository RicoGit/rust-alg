//! 36. Valid Sudoku

use std::collections::{HashMap, HashSet};
use std::ops::Range;

impl Solution {

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        Self::helper(&board)
    }

    pub fn helper(board: &[Vec<char>]) -> bool {
        let mut map = HashMap::new();
        for row_idx in 0..board.len() {
            for col_idx in 0..board[row_idx].len() {
                let cur = board[row_idx][col_idx];
                if cur == '.' {
                    continue
                }
                match map.remove(&cur) {
                    None => {
                        let mut rows = HashSet::new();
                        rows.insert(row_idx);
                        let mut colls = HashSet::new();
                        colls.insert(col_idx);
                        map.insert(cur, (rows, colls));
                    },
                    Some((mut rows, mut colls)) => {
                        if !rows.insert(row_idx) || !colls.insert(col_idx) {
                            return false
                        }
                        map.insert(cur, (rows, colls));
                    }
                }
            }
        }

        let step = 3;

        for r in 0..step {
            for c in 0..step {
                let r_start = r * step;
                let r_end = (r * step) + step;

                let c_start = c * step;
                let c_end = (c * step) + step;

                if !Self::check_square(board, r_start..r_end, c_start..c_end) {
                    return false
                }
            }
        }

        true
    }

    fn check_square(mat: &[Vec<char>], rows: Range<usize>, colls: Range<usize>) -> bool {
        let mut set = HashSet::new();
        for row in &mat[rows] {
            for el in &row[colls.clone()] {
                if *el == '.' {
                    continue
                }

                if !set.insert(*el) {
                    return false
                }
            }
        }
        true
    }

}

struct Solution;
