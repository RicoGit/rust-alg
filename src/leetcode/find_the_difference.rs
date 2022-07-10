//! 389. Find the Difference

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {

        let mut freq = s.chars().fold(std::collections::HashMap::new(), |mut acc, el| {
            *acc.entry(el).or_insert(0) += 1;
            acc
        });

        t.chars().find(|ch| {
            match freq.remove(ch) {
                None | Some(0) => true,
                Some(fr) => {
                    freq.insert(*ch, fr-1);
                    false
                }
            }
        }).unwrap()
    }
}

struct Solution;
