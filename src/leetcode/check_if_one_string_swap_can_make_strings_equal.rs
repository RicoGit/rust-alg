//! 1790. Check if One String Swap Can Make Strings Equal

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut swap = vec![];
        let str1 = s1.into_bytes();
        for (idx, ch2) in s2.bytes().enumerate() {
            if str1[idx] != ch2 { swap.push((idx, ch2)) }
            if swap.len() > 2 { return false }
        }

        if swap.len() == 0 {
            return true
        }

        if swap.len() == 2 && str1[swap[0].0] == swap[1].1 && str1[swap[1].0] == swap[0].1 {
            return true
        }

        false
    }
}

struct Solution;