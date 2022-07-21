use std::collections::VecDeque;

impl Solution {
    // iterative
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
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
                    let new_ans = [&answer[0..idx2], &[nums[idx1]], &answer[idx2..]].concat();
                    queue.push_front(new_ans)
                }
            }
        }

        queue.into_iter().collect()
    }

    // recursive (0 ms, 2.1 MB)
    pub fn permute_rec(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![vec![]];
        }

        let mut result = vec![];

        for idx in 0..nums.len() {
            let mut nums_clone = nums.clone();
            nums_clone.remove(idx);

            for mut solution in Self::permute(nums_clone) {
                solution.push(nums[idx]);
                result.push(solution)
            }
        }

        result
    }
}
struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::permute(vec![1, 2, 3]);
        assert_eq!(
            res,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![3, 1, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 2, 1]
            ]
        )
    }
}
