//! 150. Evaluate Reverse Polish Notation

use std::str::FromStr;

impl Solution {

    //  57%, 10% - bad result, make more efficient
    pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
        let mut stack: Vec<String> = vec![];
        for token in tokens.iter().rev() {
            match token.as_str() {
                operator @ ("+" | "-" | "*" | "/") => {
                    stack.push(operator.to_string());
                }
                digit => {

                    stack.push(digit.to_string());
                    while Self::is_operand(stack.get(stack.len()-2)) && stack.len() > 2 {
                        // we faced seconds operand - calculated
                        let op1 = stack.pop().unwrap();
                        let op2 = stack.pop().unwrap();
                        let operator = stack.pop().unwrap();
                        let result = Self::calc(&op1, &op2, &operator);
                        stack.push(result.to_string());
                    }
                }
            }
        }

        i32::from_str(&stack.pop().unwrap()).unwrap()
    }

    fn calc(op1: &str, op2: &str, operator: &str) -> i32 {
        let o1 =  i32::from_str(op1).unwrap();
        let o2 =  i32::from_str(op2).unwrap();
        let res = match operator {
            "+" => { o1 + o2 },
            "-" => { o1 - o2 },
            "*" => { o1 * o2 },
            "/" => { o1 / o2 },
            _ => panic!("unknown operator")
        };
        // println!("{}{}{}={}", o1, operator, o2, res);
        res
    }

    fn is_operand(str: Option<&String>) -> bool {
        if str.is_none() { return false }
        match str.unwrap().as_str() {
            "+" | "-" | "*" | "/" => false,
            _ => true,
        }
    }
}

struct Solution;