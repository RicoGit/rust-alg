//! 289. Game of Life

impl Solution {
    // live cell
    // 0 1  - dead
    // 2 3  - live
    // > 3  - dead
    //
    // dead
    // 3    - life
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let snapshot = board.clone();

        for row_idx in 0..snapshot.len() {
            for col_idx in 0..snapshot[row_idx].len() {
                if snapshot[row_idx][col_idx] == 1 {
                    match Self::number_of_neighbors(row_idx, col_idx, &snapshot) {
                        0 | 1 => board[row_idx][col_idx] = 0,
                        2 | 3 => board[row_idx][col_idx] = 1,
                        _ => board[row_idx][col_idx] = 0,
                    }
                } else {
                    if Self::number_of_neighbors(row_idx, col_idx, &snapshot) == 3 {
                        board[row_idx][col_idx] = 1;
                    }
                }
            }
        }
    }

    fn number_of_neighbors(row_idx: usize, col_idx: usize, board: &Vec<Vec<i32>>) -> usize {
        Self::cell_value(row_idx - 1, col_idx, board)
            + Self::cell_value(row_idx - 1, col_idx + 1, board)
            + Self::cell_value(row_idx - 1, col_idx - 1, board)
            + Self::cell_value(row_idx + 1, col_idx, board)
            + Self::cell_value(row_idx + 1, col_idx - 1, board)
            + Self::cell_value(row_idx + 1, col_idx + 1, board)
            + Self::cell_value(row_idx, col_idx + 1, board)
            + Self::cell_value(row_idx, col_idx - 1, board)
    }

    fn cell_value(row_idx: usize, col_idx: usize, board: &Vec<Vec<i32>>) -> usize {
        board
            .get(row_idx)
            .and_then(|r| r.get(col_idx))
            .copied()
            .unwrap_or(0) as usize
    }
}

struct Solution;
