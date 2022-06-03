//! 304. Range Sum Query 2D - Immutable

struct NumMatrix {
    cached: Vec<Vec<i32>>
}

impl NumMatrix {

    // caching rows
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut cached = matrix.clone();

        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                if col == 0 {
                    cached[row][0] = matrix[row][0];
                } else {
                    cached[row][col] = cached[row][col-1] + matrix[row][col];
                }
            }
        }
        NumMatrix{
            cached
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, row2, col1, col2) = (row1 as usize, row2 as usize, col1 as usize, col2 as usize);
        let mut sum = 0;
        for row in row1..=row2 {
            sum += self.cached[row][col2] - if col1 > 0 { self.cached[row][col1-1] } else { 0 };
        }
        sum
    }
}