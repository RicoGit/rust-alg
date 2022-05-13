//! 1641. Count Sorted Vowel Strings

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {

        // idx 0  1  2  3  4
        //
        // 1 - 1, 1, 1, 1, 1
        // 2 - 5, 4, 3, 2, 1
        // 3 - 15,10,6, 3, 1
        // 4 - 35,20,10,4, 1

        // each element corresponds 1 vowel at the beginning
        let mut result = vec![1; 5];

        for turn in 1..n {
            for i in (0..4).rev() {
                result[i] += result[i+1];
            }
        }

        result.iter().sum()
    }
}

struct Solution;
