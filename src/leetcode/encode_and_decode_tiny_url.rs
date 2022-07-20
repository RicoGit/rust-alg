//! 535. Encode and Decode TinyURL

use std::collections::HashMap;
use std::hash::Hash;
use std::time::{SystemTime, UNIX_EPOCH};

struct Codec {
    hashtable: HashMap<String, String>, // keys - short Urls
    ascii_chars: Vec<char>,
}

impl Codec {
    fn new() -> Self {
        let upper: Vec<char> = (65_u8..90_u8).into_iter().map(|c| c as char).collect();
        let lower: Vec<char> = (97_u8..122_u8).into_iter().map(|c| c as char).collect();
        let mut alfa_chars = vec![];
        alfa_chars.extend(upper);
        alfa_chars.extend(lower);
        Codec {
            hashtable: HashMap::new(),
            ascii_chars: alfa_chars,
        }
    }

    /// Encodes a URL to a shortened URL.
    /// Starts from 3-chars-length string and add new char in case of collision
    fn encode(&mut self, long_url: String) -> String {
        let mut short_url: String = (0..3)
            .into_iter()
            .map(|_| self.random_ascii_char())
            .collect();

        while let Some(existed) = self.hashtable.get(&short_url) {
            short_url.push(self.random_ascii_char())
        }

        self.hashtable.insert(short_url.clone(), long_url);
        println!("{}", short_url);
        short_url
    }

    /// Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        self.hashtable.get(&shortURL).unwrap().to_string()
    }

    /// Simple ASCII characters generator
    fn random_ascii_char(&self) -> char {
        let ns: u128 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        self.ascii_chars[ns as usize % self.ascii_chars.len()]
    }
}

struct Solution;
