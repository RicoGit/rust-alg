/// An array A consisting of N integers is given. A pair of integers (P, Q), such that 0 ≤ P ≤ Q < N,
/// is called a <i>slice</i> of array A. A slice is called <i>alternating</i> if the signs of its
/// consecutive elements alternate. More precisely, slice (P, Q) is alternating if
/// A[P] ≥ 0, A[P+1] ≤ 0, A[P+2] ≥ 0, and so on, up to A[Q], or
/// A[P] ≤ 0, A[P+1] ≥ 0, A[P+2] ≤ 0, and so on, up to A[Q].
/// Note that 0 is treated as both positive and negative.
/// Write a function:
/// class Solution { public int solution(int[] A); }
/// that, given an array A consisting of N integers, returns the size of the largest alternating slice in A.
/// For example, given array A such that:
/// A[0]  =  5
/// A[1]  =  4
/// A[2]  = -3
/// A[3]  =  2
/// A[4]  =  0
/// A[5]  =  1
/// A[6]  = -1
/// A[7]  =  0
/// A[8]  =  2
/// A[9]  = -3
/// A[10] =  4
/// A[11] = -5
/// the function should return 7, because:
/// (1, 7) is an alternating slice of length 7,
/// (7, 11) is an alternating slice of length 5, and
/// all other alternating slices in A are shorter.
/// Given array A such that:
/// A[0] = 1
/// A[1] = 2
/// A[2] = 3
/// the function should return 1, because there are three alternating slices (0, 0), (1, 1) and (2, 2) and all of them are of length 1.
/// Write an efficient algorithm for the following assumptions:
/// N is an integer within the range [1..100,000]
/// each element of array A is an integer within the range [−1,000,000,000..1,000,000,000].
///
struct Solution;

impl Solution {

    fn solution(nums: Vec<i32>) -> usize {

        fn alternate(nums: &[i32], idx1: usize, idx2: usize, positive_zero: &mut bool) -> bool {
            match (nums[idx1],  nums[idx2]) {
                (0, 0) => {
                    *positive_zero = !*positive_zero;
                    true
                },
                (0, n2) => {
                    (n2 >= 0 && !*positive_zero) || (n2 <= 0 && *positive_zero)
                },
                (n1, 0) => {
                    *positive_zero = n1 < 0;
                    true
                },
                (n1, n2) => {
                    match (n1 > 0,  n2 > 0) {
                        (false, true) | (true, false) => true,
                        (true, true) | (false, false) => false,
                    }
                }
            }
        }

        let mut dist = vec![1; nums.len()];
        let mut positive_zero = true;

        for idx in 1..nums.len() {
            if alternate(&nums, idx-1, idx, &mut positive_zero) {
                dist[idx] += dist[idx-1];
            } else {
                if nums[idx-1] == 0 {
                    positive_zero = nums[idx].is_negative();
                    dist[idx] = 2
                } else {
                    dist[idx] = 1
                }
            }
        }

        println!("{:?}", dist);

        *dist.iter().max().unwrap()
    }

}


#[cfg(test)]
mod tests {
    use crate::codility::alternate_slices::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::solution(vec![1]), 1);
        assert_eq!(Solution::solution(vec![1, 2, 3]), 1);
        assert_eq!(Solution::solution(vec![1, 0,-1, 2]), 3);
        assert_eq!(Solution::solution(vec![0, 0, 0]), 3);
        assert_eq!(Solution::solution(vec![5, 4, -3, 2, 0, 1, -1, 0, 2, -3, 4, -5]), 7);
    }
}