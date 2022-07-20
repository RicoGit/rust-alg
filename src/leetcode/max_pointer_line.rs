//! 149. Max Points on a Line

use std::cmp::Ordering::Equal;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 0 {
            return 0;
        }
        if points.len() == 1 {
            return 1;
        }

        let mut ans = 0;
        for idx1 in 0..points.len() - 1 {
            let p1 = (points[idx1][0], points[idx1][1]);
            let mut equations = HashMap::new();
            let mut same_point = 0;
            let mut same_line = 1;
            for idx2 in idx1 + 1..points.len() {
                let p2 = (points[idx2][0], points[idx2][1]);
                if p1 == p2 {
                    same_point += 1;
                    continue;
                }
                let new_eq = Equation::solve(p1, p2);

                let current = if let Some(old) = equations.remove(&new_eq) {
                    equations.insert(new_eq, old + 1);
                    old + 1
                } else {
                    equations.insert(new_eq, 2);
                    2
                };
                same_line = same_line.max(current);
            }
            // println!("{:?} {:?}", p1, equations);
            ans = ans.max(same_line + same_point);
        }

        ans
    }
}

struct Solution;

/// Corresponds to line function y = kx +b
#[derive(PartialEq, Eq, Debug, Hash)]
enum Equation {
    Vertical { x: i32 },
    Horizontal { y: i32 },
    Slope { delta_y: i32, delta_x: i32 },
}

impl Equation {
    fn solve(p1: (i32, i32), p2: (i32, i32)) -> Self {
        let (x1, y1) = p1;
        let (x2, y2) = p2;

        let mut dx = (x2 - x1);
        let mut dy = (y2 - y1);

        if dx == 0 {
            return Equation::Vertical { x: x1 };
        } else if dy == 0 {
            return Equation::Horizontal { y: y1 };
        } else {
            let g = gcd(dx, dy);
            return Equation::Slope {
                delta_y: dy / g,
                delta_x: dx / g,
            };
        }
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_points() {
        assert_eq!(
            Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            3
        );
    }

    #[test]
    fn test_max_points_02() {
        assert_eq!(
            Solution::max_points(vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4]
            ]),
            4
        );
    }
}
