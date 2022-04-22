//! 167. Two Sum II - Input Array Is Sorted

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut last_idx = numbers.len() - 1;
        for idx1 in 0..=last_idx {
            let current = numbers[idx1];

            for idx2 in (idx1 + 1)..=last_idx {
                if current + numbers[idx2] > target {
                    // reduces number of element form tail of array (optimisation)
                    last_idx = idx2 - 1;
                    break;
                }

                if current + numbers[idx2] == target {
                    return vec![1 + idx1 as i32, 1 + idx2 as i32];
                }
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::two_sum2_input_array_sorted::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3])
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::two_sum(vec![5, 25, 75], 100), vec![2, 3])
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::two_sum(vec![1, 2, 3, 4, 4, 9, 56, 90], 8),
            vec![4, 5]
        )
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::two_sum(vec![-1, -1, 1, 1, 1, 1], -2), vec![1, 2])
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::two_sum(vec![-3, 3, 4, 90], 0), vec![1, 2])
    }
}
