//!
//! The Module contains common utility methods.
//!

/// Allows to create string from repeatable values.
pub trait MkString {
    fn mk_string(self, start: &str, sep: &str, end: &str) -> String;

    fn mk_str(self, sep: &str) -> String;
}

/// Default implementation for each Iterator with elements converted to String.
impl<S, T> MkString for T
where
    S: Into<String>,
    T: IntoIterator<Item = S>,
{
    fn mk_string(self, start: &str, sep: &str, end: &str) -> String {
        let mut iterator = self.into_iter().peekable();

        if let None = iterator.peek() {
            return start.to_string() + end;
        };

        let mut result = String::new();
        let mut add_sep = false;
        result.push_str(start);

        for el in iterator {
            if add_sep {
                result.push_str(sep);
            }
            result.push_str(&el.into());
            add_sep = true;
        }
        result.push_str(end);
        result
    }

    /// Short version of `mk_string`.
    fn mk_str(self, sep: &str) -> String {
        self.mk_string("", sep, "")
    }
}

#[cfg(test)]
mod tests {
    use super::MkString;

    #[test]
    fn mk_string_is_correct() {
        let empty: Vec<String> = vec![];
        let result1 = empty.mk_string("(", ",", ")");
        assert_eq!(result1, "()".to_string());

        let single = vec!["q"];
        let result2 = single.mk_string("[", "-", "]");
        assert_eq!(result2, "[q]".to_string());

        let filled1 = vec!["1", "2", "3", "4"];
        let result3 = filled1.mk_str("+");
        assert_eq!(result3, "1+2+3+4".to_string());

        let filled2 = vec!["1", "2"];
        let result4 = filled2.mk_string("{", "", "}");
        assert_eq!(result4, "{12}".to_string());
    }
}
