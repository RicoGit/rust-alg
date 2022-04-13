//! 59. Spiral Matrix II


enum Direction {
    Right, Left, Top, Bottom
}

impl Solution {

    // 1 2
    // 4 3
    // 1243

    // 1 2 3
    // 8 9 4
    // 7 6 5

    // 1  2  3  4
    // 12 13 14 5
    // 11 16 15 6
    // 10  9 8  7
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let size = n * n;
        let mut res = vec![vec![0i32; n as usize]; n as usize];
        let mut row = 0;
        let mut col = 0;
        let mut counter: i32 = 1;
        let mut direction = Direction::Right;

        loop {
            if counter > size { break; }

            res[row][col] = counter;
            counter += 1;

            // println!("res[{}][{}]={:?}", row, col, res);

            match direction {
                Direction::Right => {
                    if !Self::has_next(row, col+1, &res) {
                        direction = Direction::Bottom;
                        row += 1;
                    } else {
                        col += 1;
                    }
                },
                Direction::Bottom => {
                    if !Self::has_next(row+1, col, &res) {
                        direction = Direction::Left;
                        col -= 1;
                    } else {
                        row += 1;
                    }
                },
                Direction::Left => {
                    if !Self::has_next(row, col-1, &res) {
                        direction = Direction::Top;
                        row -= 1;
                    } else {
                        col -= 1;
                    }
                },
                Direction::Top => {
                    if !Self::has_next(row-1, col, &res) {
                        direction = Direction::Right;
                        col += 1;
                    } else {
                        row -= 1;
                    }
                }
            }
        }

        res
    }

    fn has_next(row: usize, col: usize, vec: &Vec<Vec<i32>>) -> bool {
        vec.get(row).and_then(|r| r.get(col)).copied().unwrap_or(-1) == 0
    }
}

struct Solution;
