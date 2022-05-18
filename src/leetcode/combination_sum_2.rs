//! 40. Combination Sum II

use std::collections::HashSet;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut result = HashSet::new();
        Self::backtrack(&candidates, target, vec![], &mut result);
        result.into_iter().collect()
    }

    fn backtrack(cand: &[i32], target: i32, mut ans: Vec<i32>, result: &mut HashSet<Vec<i32>>) {
        let sum = cand.iter().sum::<i32>();
        if sum < target { return }
        if sum == target {
            ans.extend(cand);
            result.insert(ans);
            return
        }

        for (idx, num) in cand.iter().enumerate() {
            let remain = target - num;
            if remain < 0 { break }

            if idx > 0 && num == &cand[idx - 1] {
                continue
            }

            let mut new_ans = ans.clone();
            new_ans.push(*num);

            if remain == 0 { // it's an answer
                new_ans.sort();
                result.insert(new_ans);
                continue
            }

            Self::backtrack(&cand[(idx+1)..], remain, new_ans, result)
        }
    }
}


struct Solution;
