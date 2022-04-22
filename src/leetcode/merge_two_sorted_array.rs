//! 88. Merge Sorted Array

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let res = nums1;
        let m = m as usize;
        let n = n as usize;
        let nums1 = res.clone();
        res.clear();
        let mut idx1 = 0;
        let mut idx2 = 0;

        loop {
            if idx1 == m {
                res.extend_from_slice(&nums2[idx2..]);
                break;
            }

            if idx2 == n {
                res.extend_from_slice(&nums1[idx1..m]);
                break;
            }

            if nums1[idx1] < nums2[idx2] {
                res.push(nums1[idx1]);
                idx1 += 1;
            } else {
                res.push(nums2[idx2]);
                idx2 += 1;
            }
        }
    }
}

struct Solution;
