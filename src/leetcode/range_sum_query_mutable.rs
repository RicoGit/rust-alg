//! 307. Range Sum Query - Mutable
//!
//! Quite slow, will be better to use BIT (https://www.topcoder.com/thrive/articles/Binary%20Indexed%20Trees)

struct NumArray {
    nums: Vec<i32>,
    sums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let sums = nums.iter().fold(vec![], |mut acc, el| {
            let next = acc.last().unwrap_or(&0) + el;
            acc.push(next);
            acc
        });
        NumArray {
            nums,
            sums
        }
    }

    fn update(&mut self, index: i32, val: i32) {
        let start_idx = index as usize;
        let diff = self.nums[start_idx] - val;
        for idx in start_idx..self.sums.len() {
            self.sums[idx] -= diff
        }
        self.nums[start_idx] = val;

    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sums[right as usize] - *self.sums.get(left as usize - 1).unwrap_or(&0)
    }
}