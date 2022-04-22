//! 784. Letter Case Permutation

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut chars: Vec<char> = s.chars().collect();
        let mut result = vec![];
        Self::backtrack(&mut vec![], &chars, &mut result);
        result
            .into_iter()
            .map(|chars| chars.into_iter().collect::<String>())
            .collect::<Vec<String>>()
    }

    fn backtrack(solution: &mut Vec<char>, chars: &[char], result: &mut Vec<Vec<char>>) {
        if chars.is_empty() {
            result.push(solution.to_owned());
            return;
        }

        let letter = chars[0];
        let next_chars = &chars[1..];

        if letter.is_alphabetic() {
            let mut sol1 = solution.clone();
            sol1.push(letter.to_ascii_lowercase());
            Self::backtrack(&mut sol1, next_chars, result);

            let mut sol2 = solution;
            sol2.push(letter.to_ascii_uppercase());
            Self::backtrack(&mut sol2, next_chars, result);
        } else {
            solution.push(letter);
            Self::backtrack(solution, next_chars, result);
        }
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::letter_case_permutation("a1b2".to_string());
        assert_eq!(
            res,
            vec!["a1b2", "a1B2", "A1b2", "A1B2"]
                .into_iter()
                .map(|str| str.to_string())
                .collect::<Vec<String>>()
        )
    }
}
