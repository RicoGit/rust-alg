//! 473. Matchsticks to Square

impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        fn backtrack(square: &mut [i32], sticks: &[i32], idx: usize) -> bool {
            if idx == sticks.len() {
                return square.iter().all(|side| *side == 0);
            }

            for square_idx in 0..4 {
                if square[square_idx] - sticks[idx] >= 0 {
                    square[square_idx] -= sticks[idx];
                    if backtrack(square, sticks, idx + 1) {
                        return true;
                    }
                    square[square_idx] += sticks[idx];
                }
            }
            false
        }

        let perimeter = matchsticks.iter().sum::<i32>();
        if perimeter % 4 != 0 {
            return false;
        }
        let side = perimeter / 4;
        let mut square = [side; 4];
        let n = matchsticks.len();
        matchsticks.sort_by_key(|s| -s);

        backtrack(&mut square, &mut matchsticks, 0)
    }

    // eager solution doesn't work
    pub fn makesquare_wrong(mut matchsticks: Vec<i32>) -> bool {
        let perimeter = matchsticks.iter().sum::<i32>();
        if perimeter % 4 != 0 {
            return false;
        }
        let side = perimeter / 4;

        let mut answer = [side; 4];

        matchsticks.sort();
        let mut idx = 0;
        for stick in matchsticks.iter().rev() {
            let mut tries = 4;
            while tries >= 0 {
                idx += 1;
                idx %= 4;
                tries -= 1;
                if answer[idx] >= *stick {
                    answer[idx] -= stick;
                    break;
                }
            }
        }

        !answer.iter().find(|s| **s != 0).is_some()
    }
}

struct Solution;
