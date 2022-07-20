//! 745. Prefix and Suffix Search

#[derive(Default)]
struct TrieNode {
    index: i32,
    children: [Option<Box<TrieNode>>; 27],
}

struct WordFilter {
    trie: TrieNode,
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut trie = TrieNode::default();
        for (i, word) in words.iter().enumerate() {
            let s = String::new() + &word + "#" + &word;
            for j in 0..word.len() {
                let mut node = &mut trie;
                for &b in &s.as_bytes()[j..] {
                    node = node.children[(b - b'a') as usize].get_or_insert_with(Default::default);
                    node.index = i as i32;
                }
            }
        }
        Self { trie }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        let mut node = &self.trie;
        let s = String::new() + &suffix + "#" + &prefix;
        for &b in s.as_bytes() {
            if let Some(n) = &node.children[(b - b'a') as usize] {
                node = n.as_ref();
            } else {
                return -1;
            }
        }
        node.index
    }
}

mod time_limit_exceed_solution {

    use std::borrow::Borrow;
    use std::cell::{Ref, RefCell};
    use std::collections::HashSet;
    use std::rc::Rc;

    #[derive(Debug, Clone)]
    struct TrieNode {
        keys: Vec<char>,
        values: Vec<TrieNode>,
        indexes: HashSet<usize>,
    }

    impl TrieNode {
        fn empty() -> Self {
            TrieNode {
                keys: Vec::with_capacity(26),
                values: Vec::with_capacity(26),
                indexes: HashSet::new(),
            }
        }

        fn prefixes(words: Vec<String>) -> Self {
            Self::new(words, true)
        }

        fn suffixes(words: Vec<String>) -> Self {
            Self::new(words, false)
        }

        fn new(words: Vec<String>, prefixes: bool) -> Self {
            let mut root = TrieNode::empty();

            for (word_idx, word) in words.into_iter().enumerate() {
                let mut node = &mut root;
                let chars: Box<dyn Iterator<Item = char>> = if prefixes {
                    Box::new(word.chars())
                } else {
                    Box::new(word.chars().rev())
                };
                for char in chars {
                    match node.keys.binary_search(&char) {
                        Ok(idx) => {
                            node.indexes.insert(word_idx);
                            node = &mut node.values[idx]; // found: select next node
                            node.indexes.insert(word_idx);
                        }
                        Err(idx) => {
                            node.keys.insert(idx, char);
                            node.values.insert(idx, TrieNode::empty());
                            node.indexes.insert(word_idx);
                            node = &mut node.values[idx]; // select next node
                            node.indexes.insert(word_idx);
                        }
                    }
                }
            }
            root
        }
    }

    #[derive(Debug, Clone)]
    struct WordFilter {
        prefixes: TrieNode,
        suffixes: TrieNode,
    }

    impl WordFilter {
        fn new(words: Vec<String>) -> Self {
            let prefixes = TrieNode::prefixes(words.clone());
            let suffixes = TrieNode::suffixes(words);

            WordFilter { prefixes, suffixes }
        }

        fn f(&mut self, prefix: String, suffix: String) -> i32 {
            let mut found_by_prefix = None;
            let mut node = &mut self.prefixes;
            for char in prefix.chars() {
                match node.keys.binary_search(&char) {
                    Ok(idx) => {
                        node = &mut node.values[idx];
                        found_by_prefix = Some(&node.indexes);
                    }
                    Err(_) => {
                        found_by_prefix = None;
                        break;
                    }
                }
            }

            let found_by_prefix = found_by_prefix.unwrap().clone();

            let mut found_by_suffix = None;
            let mut node = &mut self.suffixes;
            for char in suffix.chars().rev() {
                match node.keys.binary_search(&char) {
                    Ok(idx) => {
                        node = &mut node.values[idx];
                        found_by_suffix = Some(&node.indexes);
                    }
                    Err(_) => {
                        found_by_suffix = None;
                        break;
                    }
                }
            }
            let found_by_suffix = found_by_suffix.unwrap().clone();

            found_by_suffix
                .intersection(&found_by_prefix)
                .max()
                .map(|r| *r as i32)
                .unwrap_or(-1)
        }
    }

    struct Solution;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut filter = WordFilter::new(vec!["apple".to_string()]);
        let res = filter.f("ap".to_string(), "e".to_string());
        assert_eq!(res, 0)
    }

    #[test]
    fn test2() {
        let mut filter = WordFilter::new(
            vec![
                "cabaabaaaa",
                "ccbcababac",
                "bacaabccba",
                "bcbbcbacaa",
                "abcaccbcaa",
                "accabaccaa",
                "cabcbbbcca",
                "ababccabcb",
                "caccbbcbab",
                "bccbacbcba",
            ]
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        );
        assert_eq!(filter.f("bccbacbcba".to_string(), "a".to_string()), 9);
        assert_eq!(filter.f("ab".to_string(), "abcaccbcaa".to_string()), 4);
        assert_eq!(filter.f("a".to_string(), "aa".to_string()), 5);
    }

    #[test]
    fn test3() {
        let mut filter = WordFilter::new(
            vec!["a", "a", "a", "a", "a", "a", "a", "b", "b", "b"]
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        );
        let res = filter.f("a".to_string(), "a".to_string());
        assert_eq!(res, 6)
    }
}
