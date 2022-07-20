//! 1356. Sort Integers by The Number of 1 Bits

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut result = arr
            .into_iter()
            .map(|n| (n.count_ones(), n))
            .collect::<Vec<_>>();
        result.sort();
        result.into_iter().map(|p| p.1).collect::<Vec<_>>()
    }
}

struct Solution;
