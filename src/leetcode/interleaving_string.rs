//! 97. Interleaving String

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() { return false }
        let mut buf = vec![vec![None; s2.len()+1]; s1.len()+1];

        #[inline]
        fn helper(s1: &[u8], s2: &[u8], s3: &[u8], buf: &mut Vec<Vec<Option<bool>>> ) -> bool {
            if s3.is_empty() { return s1.len() + s2.len() == 0 }
            if s1.is_empty() { return s2 == s3 }
            if s2.is_empty() { return s1 == s3 }
            if let Some(res) = buf[s1.len()][s2.len()] { return res }
            buf[s1.len()][s2.len()] = Some(s1[0] == s3[0] && helper(&s1[1..], s2, &s3[1..], buf) || s2[0] == s3[0] && helper(s1, &s2[1..], &s3[1..], buf));
            buf[s1.len()][s2.len()].unwrap()
        }

        helper(&s1.into_bytes(), &s2.into_bytes(), &s3.into_bytes(), &mut buf)
    }
}

struct Solution;