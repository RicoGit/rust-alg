//! 682. Baseball Game


use std::str::FromStr;

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {

        let mut res: Vec<i32> = vec![];

        for op in ops {
            match op.as_str() {
                "D" => {
                    
                },
                "C" => {


                },
                "+" => {
                    let prev = res[res.len()-2].clone();
                    let last = res[res.len()-1].clone();
                    res.push(prev+last)
                },
                num => res.push(i32::from_str(num).unwrap())
            }
        }

        res.into_iter().sum()
    }
}

struct Solution;
