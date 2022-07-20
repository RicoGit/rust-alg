//! 1689. Partitioning Into Minimum Number Of Deci-Binary Numbers

impl Solution {
    // oneliner
    pub fn min_partitions(n: String) -> i32 {
        (n.bytes().max().unwrap() - b'0') as i32
    }

    // simple
    pub fn min_partition_(n: String) -> i32 {
        let mut max = u8::MIN;
        for digit in n.bytes() {
            max = max.max(digit);
        }
        (max - b'0') as i32
    }
}
struct Solution;
