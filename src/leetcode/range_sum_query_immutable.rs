//! 303. Range Sum Query - Immutable

struct NumArray {
    sums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sums = vec![0; nums.len() + 1];
        for idx in 0..nums.len() {
            sums[idx + 1] = sums[idx] + nums[idx];
        }
        NumArray { sums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sums[right as usize + 1] - self.sums[left as usize]
    }
}

struct Solution;
