//! 1678. Goal Parser Interpretation

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut result = String::new();
        for idx in 0..command.len() {
            let cur = &command[idx..=idx];
            match cur {
                "G" => { result += "G"; },
                "(" => {
                    match &command[idx+1..=idx+1] {
                        ")" => { result += "o"; },
                        "a" => { result += "al"; },
                        _ => ()
                    }
                },
                _ => {}
            }
        }

        result
    }
}

struct Solution;