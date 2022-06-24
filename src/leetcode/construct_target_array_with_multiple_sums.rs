//! 1354. Construct Target Array With Multiple Sums

impl Solution {

    pub fn is_possible(target: Vec<i32>) -> bool {
        use std::collections::BinaryHeap;

        let mut sum = 0i64;
        let mut heap = BinaryHeap::<i64>::new();
        for x in target {
            let x = x as i64;
            heap.push(x);
            sum += x;
        }
        while let Some(mut x) = heap.pop() {
            let mut x = x as i64;
            sum -= x;
            if x == 1 || sum == 1 {
                return true;
            }
            if sum >= x || sum == 0  {
                return false;
            }
            x = x % sum; // reduces number of iterations
            sum += x;
            heap.push(x);
        }
        false
    }
}

struct Solution;