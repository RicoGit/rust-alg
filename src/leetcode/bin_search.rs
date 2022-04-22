//! 704. Binary Search

struct Solution();

impl Solution {
    // Constrains:
    //
    // 1 <= nums.length <= 104
    // -104 < nums[i], target < 104
    // All the integers in nums are unique.
    // nums is sorted in ascending order.
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        search_rec(&nums, 0, nums.len() - 1, target)
    }
}

fn search_rec(nums: &[i32], start: usize, end: usize, target: i32) -> i32 {
    if end == start {
        if nums[start] == target {
            return start as i32;
        } else {
            return -1;
        }
    }

    let mid = dbg!((start / 2) + (end / 2));

    if target <= nums[mid] {
        search_rec(nums, start, mid, target)
    } else {
        search_rec(nums, mid + 1, end, target)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::search(vec![5], -5), -1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::search(vec![-1, 0, 5], 0), 1);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
