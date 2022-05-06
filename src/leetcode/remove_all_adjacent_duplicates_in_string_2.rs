//! 1209. Remove All Adjacent Duplicates in String II

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let k = k as usize;
        let str = s.as_bytes();
        let mut stack: Vec<(u8, usize)> = vec![];

        for byte in str {
            if let Some((cur_byte, counter)) = stack.pop() {
                if cur_byte == *byte {
                    if counter+1 < k {
                        stack.push((cur_byte, counter+1))
                    }
                } else {
                    stack.push((cur_byte, counter));
                    stack.push((*byte, 1));
                }
            } else {
                stack.push((*byte, 1))
            }
        }

        String::from_utf8(stack.into_iter().flat_map(|(ch, num)| std::iter::repeat(ch).take(num as usize)).collect::<Vec<_>>()).unwrap()
    }
}

struct Solution;
