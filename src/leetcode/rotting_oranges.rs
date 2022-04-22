//! 994. Rotting Oranges

use std::collections::LinkedList;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let colls = grid[0].len();

        // let mut mat = vec![vec![42;colls];rows]
        let mut grid = grid;

        // contains only rotten oranges
        let mut queue = LinkedList::<(usize, usize)>::new();

        for r in 0..rows {
            for c in 0..colls {
                if grid[r][c] == 2 {
                    queue.push_back((r, c));
                }
            }
        }

        let neighbours = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some((r, c)) = queue.pop_front() {
            let current = grid[r][c];

            for (row_delta, coll_delta) in neighbours.iter() {
                let new_r = row_delta + r as i32;
                let new_c = coll_delta + c as i32;

                // println!("row: {}+{}={}", row_delta, r, new_r);
                // println!("coll: {}+{}={}", coll_delta, c, new_c);

                if new_r < 0 || new_r > rows as i32 - 1 || new_c < 0 || new_c > colls as i32 - 1 {
                    continue;
                }

                let new_r = new_r as usize;
                let new_c = new_c as usize;

                if grid[new_r][new_c] == 0 {
                    continue;
                }

                let current = grid[r][c];
                let new = grid[new_r][new_c];

                if current > new {
                    if new == 1 {
                        grid[new_r][new_c] = current + 1;
                        queue.push_back((new_r, new_c));
                    } else {
                        grid[r][c] = new + 1;
                    }
                } else if current < new {
                    grid[new_r][new_c] = current + 1;
                    queue.push_back((new_r, new_c));
                }
            }
            // println!("{:?}{:?}", grid, queue);
        }

        let mut max = i32::MIN;
        for r in 0..rows {
            for c in 0..colls {
                if grid[r][c] == 1 {
                    return -1;
                }
                max = max.max(grid[r][c]);
            }
        }
        if max > 0 {
            return max - 2;
        } else {
            0
        }
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]);
        assert_eq!(res, 4)
    }
}
