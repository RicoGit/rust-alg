//! 39. Combination Sum

use std::collections::HashSet;

impl Solution {

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = HashSet::new();
        Self::helper(&candidates, target, vec![], &mut result);
        result.into_iter().collect()
    }

    fn helper(cand: &[i32], target: i32, mut ans: Vec<i32>, result: &mut HashSet<Vec<i32>>) {
        for (idx, num) in cand.iter().enumerate() {
            let rest = target - num;
            if rest >= 0 {
                let mut ans = ans.clone();
                ans.push(*num);

                if rest == 0 {
                    ans.sort();
                    result.insert(ans);
                    continue
                }

                Self::helper(&cand[idx..], rest, ans, result) // cand[idx..] is important optimisation
            }
        }
    }
}

struct Solution;
