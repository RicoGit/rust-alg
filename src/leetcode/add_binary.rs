//! 67. Add Binary

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let res = i128::from_str_radix(&a, 2).unwrap() + i128::from_str_radix(&b, 2).unwrap();
        format!("{:b}", res)
    }
}

struct Solution;
