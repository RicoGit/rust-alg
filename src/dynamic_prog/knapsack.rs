//! The knapsack problem or rucksack problem is a problem in combinatorial optimization:
//! Given a set of items, each with a weight and a value, determine the number of each item to
//! include in a collection so that the total weight is less than or equal to a given limit and
//! the total value is as large as possible. It derives its name from the problem faced by someone
//! who is constrained by a fixed-size knapsack and must fill it with the most valuable items.
#![allow(unused)]

use std::cmp::max;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Item {
    weight: usize,
    value: usize,
}

impl Item {
    fn new(weight: usize, value: usize) -> Self {
        Item { weight, value }
    }
}

#[derive(Debug)]
struct Matrix {
    items: usize,
    weights: usize,
    buffer: Vec<usize>,
}

impl Matrix {
    fn new(rows: usize, columns: usize) -> Self {
        Matrix {
            items: rows,
            weights: columns,
            buffer: vec![0; rows * columns],
        }
    }

    fn get(&self, row: usize, col: usize) -> usize {
        self.buffer[self.offset(row, col)]
    }

    fn get_row(&self, row: usize) -> &[usize] {
        &self.buffer[(row - 1) * self.weights..(row * self.weights)]
    }

    fn set(&mut self, row: usize, col: usize, value: usize) {
        let offset = self.offset(row, col);
        self.buffer[offset] = value;
    }

    fn offset(&self, row: usize, col: usize) -> usize {
        if row == 0 {
            col
        } else {
            row * self.weights + col
        }
    }

    fn result_value(&self) -> usize {
        self.get(self.items - 1, self.weights - 1)
    }
}

pub fn solve(items: Vec<Item>, knapsack_capacity: usize) -> usize {
    let matrix = Matrix::new(items.len() + 1, knapsack_capacity);

    fn opt(idx: usize, item: &Item, weight_left: usize, matrix: &Matrix) -> usize {
        if weight_left < item.weight {
            // if this item can't be added, just return calculated value for prev item
            matrix.get(idx - 1, weight_left)
        } else {
            // if this item could be added, choose between adding it and leaving the previous one
            max(
                matrix.get(idx - 1, weight_left),
                item.value + matrix.get(idx - 1, weight_left - item.weight),
            )
        }
    }

    let result_matrix = items
        .iter()
        .enumerate()
        .fold(matrix, move |mut acc, (idx, item)| {
            for weight in 0..knapsack_capacity {
                let value = opt(idx + 1, item, weight, &acc);
                acc.set(idx + 1, weight, value);
            }
            acc
        });

    result_matrix.result_value()

    // todo return selected items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_success() {
        let items = vec![
            Item::new(1, 2),
            Item::new(2, 1),
            Item::new(3, 5),
            Item::new(3, 3),
        ];
        let solution = solve(items, 5);
        assert_eq!(solution, 7)
    }

    // todo add prop based testing
}
