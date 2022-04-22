use std::cmp::Reverse;
use std::collections::{BinaryHeap, LinkedList};
use std::iter::FromIterator;

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let heap = BinaryHeap::from_iter(nums.into_iter().map(|e| Reverse(e)));
        let mut res = KthLargest {
            heap,
            k: k as usize,
        };
        res.shrink();
        res
    }

    fn add(&mut self, val: i32) -> i32 {
        if let Some(min) = self.heap.peek().map(|e| e.0) {
            let result = if min < val {
                self.heap.pop().unwrap().0;
                self.heap.push(Reverse(val));
                self.heap.peek().unwrap().0
            } else {
                min
            };
            return result;
        } else {
            -1
        }
    }

    fn shrink(&mut self) {
        if self.heap.len() > self.k {
            self.heap.pop();
            self.shrink()
        }
    }
}
