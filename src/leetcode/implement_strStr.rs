//! 28. Implement strStr()

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() || needle.len() > haystack.len() { return -1 };
        let needle_bytes = needle.into_bytes();
        let haystack_bytes = haystack.into_bytes();

        for (idx, &byte) in haystack_bytes.iter().enumerate() {
            if byte == needle_bytes[0] && Self::is_eq(&haystack_bytes[idx..], &needle_bytes) {
                return idx as i32
            }
        }
        -1
    }

    #[inline]
    fn is_eq(arr1: &[u8], arr2: &[u8]) -> bool {
        if arr1.len() < arr2.len() { return false };
        for idx in 1..arr2.len() {
            if arr1[idx] != arr2[idx] {
                return false
            }
        }
        true
    }
}

struct Solution;