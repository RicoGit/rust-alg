//! 1696. Jump Game VI

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (nums.len(), k as usize);
        let mut score = nums[n - 1];
        let mut heap = std::collections::BinaryHeap::with_capacity(n);

        heap.push((score, n - 1));
        for index in (0..(n - 1)).rev() {
            let mut score = nums[index] + loop {
                if let Some(&(val, idx)) = heap.peek() {
                    if idx < index + k {
                        heap.pop();
                        continue
                    }
                    break val;
                }
            };
            heap.push((score, index));
        }
        score
    }
}
struct Solution;
