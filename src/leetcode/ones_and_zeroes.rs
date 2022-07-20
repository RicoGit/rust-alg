//! 474. Ones and Zeroes

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let counts = strs
            .into_iter()
            .map(|str| {
                let zeroes = str.chars().filter(|ch| *ch == '0').count();
                (zeroes as i32, (str.len() - zeroes) as i32)
            })
            .collect::<Vec<_>>();

        let mut buf = vec![vec![vec![0; (n + 1) as usize]; (m + 1) as usize]; counts.len()];

        Self::helper(&counts, 0, m, n, &mut buf)
    }

    fn helper(
        counts: &[(i32, i32)],
        idx: usize,
        m: i32,
        n: i32,
        buf: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        if m < 0 || n < 0 {
            return -100;
        }
        if idx == counts.len() {
            return 0;
        }

        let (zeroes, ones) = counts[idx];
        let candidate1 = Self::helper(counts, idx + 1, m, n, buf);
        let candidate2 = Self::helper(counts, idx + 1, m - zeroes, n - ones, buf);
        buf[idx][m as usize][n as usize] = candidate1.max(candidate2 + 1);
        buf[idx][m as usize][n as usize]
    }
}

struct Solution;
