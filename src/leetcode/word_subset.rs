//! 916. Word Subsets

struct Solution;

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {

        let mut target_cache = vec![0; 26];
        for word in &words2 {
            let freq = Self::freq(word);
            for idx in 0..freq.len() {
                target_cache[idx] = target_cache[idx].max(freq[idx])
            }
        }

        words1.into_iter()
            .filter(|w| Self::is_subset(&Self::freq(w), &target_cache))
            .collect::<Vec<_>>()
    }


    fn is_subset(src: &[usize], trg: &[usize]) -> bool {
        for idx in 0..trg.len() {
            if src[idx] < trg[idx] {
                return false
            }
        }
        true
    }

    fn freq(word: &str) -> Vec<usize> {
        let mut arr = vec![0; 26];
        for ch in word.chars() {
            let idx = (ch as u8 - b'a') as usize;
            arr[idx] += 1;
        }
        arr
    }
}