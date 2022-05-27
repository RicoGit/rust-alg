//! 32. Longest Valid Parentheses

impl Solution {

    // very smart solution
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if s.is_empty() { return 0 }
        let chars: Vec<char> = s.chars().collect();
        let mut max = usize::MIN;

        let mut left = 0;
        let mut right = 0;
        for char in &chars {
            if *char == '(' { left += 1; }
            if *char == ')' { right += 1; }
            if left == right {
                max = max.max(right * 2);
            } else if right > left {
                left = 0;
                right = 0;
            }
        }

        let mut left = 0;
        let mut right = 0;
        for char in chars.iter().rev() {
            if *char == '(' { left += 1; }
            if *char == ')' { right += 1; }
            if left == right {
                max = max.max(right * 2);
            } else if left > right {
                left = 0;
                right = 0;
            }
        }

        max as i32
    }


    // dynamic programming
    pub fn longest_valid_parentheses_dp(s: String) -> i32 {
        if s.is_empty() { return 0 }

        let chars: Vec<char> = s.chars().collect();
        let mut buf = vec![0; chars.len()];

        for start in 0..chars.len() {
            let mut open = 0;
            let mut cur_len = 0;
            for idx in start..chars.len() {
                match chars[idx] {
                    '(' => open += 1,
                    ')' => {
                        open -= 1;

                        if open < 0 {
                            open = 0;
                            cur_len = 0;
                        } else {
                            cur_len += 2;
                            if open == 0 {
                                buf[idx] = buf[idx].max(cur_len);
                            }
                        }
                    },
                    _ => ()
                }
            }
        }

        println!("{:?}", buf);
        *buf.iter().max().unwrap()
    }

}

struct Solution;
