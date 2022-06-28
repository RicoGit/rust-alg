//! 1647. Minimum Deletions to Make Character Frequencies Unique

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut counts = vec![0; 26];

        for letter in s.bytes() {
            let idx = (letter - b'a') as usize;
            counts[idx] = counts[idx] + 1;
        }

        counts.sort();
        let mut set = std::collections::HashSet::<i32>::new();
        let mut result = 0;

        for freq in &mut counts {
            while *freq > 0 && !set.insert(*freq) {
                *freq -= 1;
                result += 1;
            }
        }

        result
    }
}

struct Solution;