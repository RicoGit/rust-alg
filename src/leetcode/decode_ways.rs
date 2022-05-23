//! 91. Decode Ways

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.len() < 1 {
            return 0;
        }
        let mut row: Vec<u32> = vec![0; s.len() + 1];
        row[0] = 1;

        if s.chars().nth(0).unwrap() == '0' {
            row[1] = 0
        } else {
            row[1] = 1
        };
        for idx in 2..(s.len() + 1) {
            let cur: u32 = (&s[idx - 1..idx]).parse().unwrap();
            if cur > 0 && cur <= 9 {
                row[idx] += row[idx - 1]
            }
            let prev_and_cur: u32 = (&s[idx - 2..idx]).parse().unwrap();
            if prev_and_cur > 9 && prev_and_cur <= 26 {
                row[idx] += row[idx - 2]
            }
        }
        row[s.len()] as i32
    }

    // couldn't solve
    pub fn num_decodings_(s: String) -> i32 {
        if s.len() == 1 && s != "0" {
            return 1;
        }
        let bytes = s
            .chars()
            .map(|ch| u32::from_str_radix(&ch.to_string(), 10).unwrap())
            .collect::<Vec<_>>();
        let mut counter = 0;
        let mut prev_used = false;
        for idx in (0..bytes.len() - 1).rev() {
            let cur = bytes[idx];
            if cur == 0 {
                continue;
            }
            counter += 1;
            let prev = bytes[idx + 1];
            if (prev == 1 || prev == 2) && prev_used {
                prev_used = false;
                continue;
            }
            if (cur == 1 || cur == 2) && prev < 7 && prev > 0 {
                counter += 1;
                prev_used = true;
            }
        }
        counter
    }
}

struct Solution;
