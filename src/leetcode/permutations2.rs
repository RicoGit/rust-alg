//! 47. Permutations II

use std::collections::{HashSet, VecDeque};

impl Solution {
    //recursive
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(nums: Vec<i32>) -> Vec<Vec<i32>> {
            if nums.is_empty() {
                return vec![vec![]];
            }

            let mut result = vec![];
            let mut prev = i32::MIN;

            for idx in 0..nums.len() {
                let cur = nums[idx];
                if cur != prev {
                    prev = cur;
                    let mut nums_clone = nums.clone();
                    nums_clone.remove(idx);
                    for mut solution in dfs(nums_clone) {
                        solution.push(cur);
                        result.push(solution);
                    }
                }
            }

            result
        }

        nums.sort();
        dfs(nums)
    }

    // iterative
    pub fn permute_unique_iter(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        if nums.is_empty() {
            return vec![vec![]];
        }

        let mut queue = VecDeque::new();
        queue.push_back(vec![]);

        // for each number except first
        for idx1 in 0..n {
            // for each solutions with size
            while queue.back().unwrap().len() == idx1 {
                let answer = queue.pop_back().unwrap();
                for idx2 in 0..=answer.len() {
                    if idx2 != answer.len() && answer[idx2] == nums[idx1] {
                        continue; // optimization
                    }

                    let new_ans = [&answer[0..idx2], &[nums[idx1]], &answer[idx2..]].concat();
                    queue.push_front(new_ans)
                }
            }
        }

        let set = queue.into_iter().collect::<HashSet<Vec<_>>>();
        set.into_iter().collect()
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::permute_unique(vec![1, 1, 2]);
        assert_eq!(res, vec![vec![1, 2, 1], vec![1, 1, 2], vec![2, 1, 1]])
    }
}
