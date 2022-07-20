//! 22. Generate Parentheses

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        Self::backtrack(&mut result, &mut vec![], n as usize, 0, 0);
        result
    }

    fn backtrack(
        result: &mut Vec<String>,
        mut answer: &mut Vec<char>,
        n: usize,
        open: usize,
        close: usize,
    ) {
        if answer.len() == n * 2 {
            result.push(answer.clone().into_iter().collect())
        }

        if open < n {
            answer.push('(');
            Self::backtrack(result, answer, n, open + 1, close);
            answer.remove(answer.len() - 1);
        }

        if close < open {
            answer.push(')');
            Self::backtrack(result, answer, n, open, close + 1);
            answer.remove(answer.len() - 1);
        }
    }
}

struct Solution;
