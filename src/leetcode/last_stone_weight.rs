//! 1046. Last Stone Weight

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        let mut heap = stones.into_iter().collect::<BinaryHeap<_>>();

        while heap.len() > 1 {
            let diff = heap.pop().unwrap() - heap.pop().unwrap();
            if diff > 0 {
                heap.push(diff);
            }
        }
        heap.pop().unwrap_or_default()
    }
}

struct Solution;
