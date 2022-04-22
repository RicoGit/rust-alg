//! 31. Next Permutation

pub fn next_permutation_rec(nums: &mut [i32]) {
    let mut changed_idx: Option<usize> = None;

    for idx in (1..nums.len()).rev() {
        let last = nums[idx];
        let before_last = nums[idx - 1];

        if before_last < last {
            // find idx of smaller from the end

            let idx2 = (idx..nums.len())
                .rev()
                .find(|&i| before_last < nums[i])
                .unwrap();

            swap(nums, idx - 1, idx2);
            changed_idx = Some(idx);
            break;
        }
    }

    match changed_idx {
        None => revert(nums),
        Some(idx) => revert(&mut nums[idx..]),
    }
}

fn swap(nums: &mut [i32], idx1: usize, idx2: usize) {
    let tmp = nums[idx1];
    nums[idx1] = nums[idx2];
    nums[idx2] = tmp;
}

fn revert(nums: &mut [i32]) {
    for idx in 0..(nums.len() / 2) {
        swap(nums, idx, nums.len() - 1 - idx)
    }
}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        next_permutation_rec(nums)
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use crate::leetcode::next_permutation::Solution;

    #[test]
    fn test0() {
        let mut input = vec![1, 2];
        Solution::next_permutation(&mut input);
        assert_eq!(input, vec![2, 1])
    }

    #[test]
    fn test1() {
        let mut input = vec![1, 2, 3];
        Solution::next_permutation(&mut input);
        assert_eq!(input, vec![1, 3, 2])
    }

    #[test]
    fn test2() {
        let mut input = vec![3, 2, 1];
        Solution::next_permutation(&mut input);
        assert_eq!(input, vec![1, 2, 3])
    }

    #[test]
    fn test3() {
        let mut input = vec![1, 1, 5];
        Solution::next_permutation(&mut input);
        assert_eq!(input, vec![1, 5, 1])
    }

    #[test]
    fn test4() {
        let mut input = vec![3, 4, 2, 1];
        Solution::next_permutation(&mut input);
        assert_eq!(input, vec![4, 1, 2, 3])
    }

    #[test]
    fn test5() {
        let mut input = vec![1, 3, 2];
        Solution::next_permutation(&mut input);
        assert_eq!(input, vec![2, 1, 3])
    }
}
