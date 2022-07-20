//! 79. Word Search

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let chars = word.chars().into_iter().collect::<Vec<_>>();
        for row in 0..board.len() {
            for col in 0..board[0].len() {
                if Self::backtrack(&board, &chars, row, col) {
                    return true;
                }
            }
        }
        false
    }

    fn backtrack(board: &[Vec<char>], word: &[char], row: usize, col: usize) -> bool {
        if word.is_empty() {
            return true;
        }

        let next = &word[0];
        let cur = &board[row][col];

        if cur == next {
            let remain = &word[1..];
            if row > 0 {
                return Self::backtrack(board, remain, row - 1, col);
            }

            if row < board.len() - 1 {
                return Self::backtrack(board, remain, row + 1, col);
            }

            if col > 0 {
                return Self::backtrack(board, remain, row, col - 1);
            }

            if col < board[0].len() - 1 {
                return Self::backtrack(board, remain, row, col + 1);
            }
        }

        false
    }
}

struct Solution;
