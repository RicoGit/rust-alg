//! 905. Sort Array By Parity

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        two_pointers(&mut nums);
        nums
    }
}

/// 2 pointers time: O(n), space O(1)
fn two_pointers(nums: &mut Vec<i32>) {
    let mut start = 0;
    let mut end = nums.len()-1;

    while start < end {
        if nums[start]%2 > nums[end]%2 {
            swap(nums, start, end)
        }
        start += 1;
        end -= 1;
    }
}

/// time: O(n log n), space O(1)
fn buuble_sort(nums: &mut Vec<i32>) {
    for idx1 in 0..nums.len() {
        for idx2 in idx1..nums.len() {
            if is_odd(&nums, idx1) {
                swap(nums, idx1, idx2)
            }
        }
    }
}

fn is_odd(nums: &Vec<i32>, idx: usize) -> bool {
    nums[idx] % 2 != 0
}

fn swap(nums: &mut Vec<i32>, idx1: usize, idx2: usize) {
    let tmp = nums[idx1];
    nums[idx1] = nums[idx2];
    nums[idx2] = tmp;
}

struct Solution;
