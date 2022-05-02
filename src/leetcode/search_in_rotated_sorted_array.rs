//! 33. Search in Rotated Sorted Array

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let size = nums.len();

        let pivot = if nums[0] < nums[size-1] {
            0
        } else {
            Self::find_pivot(&nums, 0, size-1)
        };
        println!("pivot {:?}", pivot);

        if let Ok(idx) = (&nums[..pivot]).binary_search(&target) {
            return idx as i32;
        }

        if let Ok(idx) = (&nums[pivot..]).binary_search(&target) {
            return (idx+pivot) as i32;
        }

        -1
    }

    fn find_pivot(nums: &Vec<i32>, start: usize, end: usize) -> usize {
        if start == end {
            return start + 1
        }
        let mid = (start+end)/2;
        println!("{} {} {}", start, mid, end);
        if nums[start] < nums[mid] {
            Self::find_pivot(nums, mid, end)
        } else {
            Self::find_pivot(nums, start, mid)
        }
    }

}

struct Solution;


#[cfg(test)]
mod tests {
    use crate::leetcode::search_in_rotated_sorted_array::Solution;

    #[test]
    fn test1() {
        let res = Solution::search(vec![1, 3], 1);
        assert_eq!(res, 0)
    }

    #[test]
    fn test2() {
        let res = Solution::search(vec![3, 1], 1);
        assert_eq!(res, 1)
    }
    #[test]
    fn test3() {
        let res = Solution::search(vec![3, 1], 3);
        assert_eq!(res, 0)
    }
    #[test]
    fn test4() {
        let res = Solution::search(vec![4,5,6,7,0,1,2], 0);
        assert_eq!(res, 4)
    }
    #[test]
    fn test5() {                      //0 1 2 3 4 5 6 7
        let res = Solution::search(vec![2,3,4,5,6,7,8,1], 8);
        assert_eq!(res, 6)
    }


}
