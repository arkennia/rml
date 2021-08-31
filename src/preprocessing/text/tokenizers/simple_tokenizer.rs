// Copyright 2021 Jonathan Manly.

// This file is part of rml.

// rml is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// rml is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

/*!
Simple Tokenizer that implements the `Tokenize` trait. This tokenizer first removes all
punctuation, and then splits the rest into word chunks. There are some sacrifices made in
regex to increase accuracy of tokenization, at the detriment to throughput.

# Example
```rust
use rml::preprocessing::text::tokenizers;
use rml::preprocessing::text::tokenizers::Tokenize;
let mut st = tokenizers::SimpleTokenizer::new(100, true);
st.create_tokens(&vec![
    String::from("Hello, my name is bob!"),
    String::from("Beep boop I'm a bot"),
    String::from("Beep boop I'm a bob!"),
]);
let mut t = st.get_tokens();
t.sort_unstable();
let mut test_data = vec![
    "UNK", "beep", "bob", "a", "my", "im", "boop", "hello", "name", "is", "bot",
];
test_data.sort_unstable();
assert_eq!(t, test_data);
```
*/

use crate::preprocessing::text::regexes;
use crate::preprocessing::text::tokenizers;

use std::collections::HashMap;

// The unknown token string.
const UNKNOWN_STR: &str = "UNK";
// The index of the unknown token.
const UNKNOWN_IDX: usize = 0;

/**
Contains the data and options for the tokenizer.
*/
#[derive(Debug, Clone)]
pub struct SimpleTokenizer {
    /// Total number of tokens to create. Set to -1 to keep all.
    pub max_tokens: i32,
    /// Make tokens lowercase.
    pub use_lowercase: bool,
    /// The tokens generated and their index in the frequency vector.
    tokens: HashMap<String, (usize, u32)>,
}

impl Default for SimpleTokenizer {
    fn default() -> Self {
        Self {
            max_tokens: 10,
            use_lowercase: true,
            tokens: Default::default(),
        }
    }
}

impl SimpleTokenizer {
    pub fn new(max_tokens: i32, use_lowercase: bool) -> Self {
        Self {
            max_tokens,
            use_lowercase,
            ..Self::default()
        }
    }
}

impl tokenizers::Tokenize for SimpleTokenizer {
    /**
        Create the tokens to use for tokenization of a text.
        It stores the created tokens internally, and can be retrieved wit the `get_tokens` function.
    */
    fn create_tokens(&mut self, data: &[String]) {
        let mut hashmap: HashMap<String, (usize, u32)> = HashMap::new();
        for entry in data {
            for x in self.sanitize_line(entry.trim().to_string()).split(' ') {
                let tmp = hashmap.insert(x.to_string(), (hashmap.len() + 1, 1));
                if let Some(y) = tmp {
                    hashmap.insert(x.to_string(), (y.0, y.1 + 1));
                }
            }
        }
        hashmap.remove("");
        if self.max_tokens > 0 {
            // Get the keys from the hashmap.
            let mut token_keys: Vec<&String> = hashmap.keys().collect();
            // Sort them by frequency.
            token_keys.sort_by(|a, b| hashmap.get(*b).unwrap().1.cmp(&hashmap.get(*a).unwrap().1));
            // Limit max_tokens.
            token_keys = token_keys
                .into_iter()
                .take(self.max_tokens as usize)
                .collect();
            let mut hashmap = hashmap.to_owned();
            hashmap.retain(|x, _b| token_keys.contains(&x));
            // Add the unknown token to the hashmap of tokens.
            hashmap.insert(UNKNOWN_STR.to_string(), (UNKNOWN_IDX, 0));

            // Move hashmap to the tokenizer.
            self.tokens = hashmap;
        } else {
            hashmap.insert(UNKNOWN_STR.to_string(), (UNKNOWN_IDX, 0));

            // Move hashmap to the tokenizer.
            self.tokens = hashmap;
        }
    }

    /**
    Turn the given string into a vector of integers matching the corrrect feature,
    or place a 0 for unknown tokens.

    # Note
    If the `create_tokens` function was not called before this one, it will return none.
    */
    fn encode(&self, input: &str) -> Option<Vec<i32>> {
        if !self.tokens.is_empty() {
            let input = input.to_owned();
            let mut output: Vec<i32> = Vec::default();
            for x in self.sanitize_line(input.trim().to_string()).split(' ') {
                output.push(self.tokens.get(x).unwrap_or(&(UNKNOWN_IDX, 0)).0 as i32);
            }
            Some(output)
        } else {
            None
        }
    }

    /**
    Turn the given integer slice into a string matching the corrrect features,
    or place an `UNK` token for unknowns.

    # Note
    If the `create_tokens` function was not called before this one, it will return none.
    */
    fn decode(&self, input: &[i32]) -> Option<String> {
        if !self.tokens.is_empty() {
            let mut output = String::new();
            for word in input {
                output.push_str(
                    &self
                        .tokens
                        .iter()
                        .find_map(|(k, val)| {
                            if val.0 == *word as usize {
                                Some(k.to_string())
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| UNKNOWN_STR.to_string()),
                );
                output.push(' ');
            }
            Some(output.trim().to_string())
        } else {
            None
        }
    }

    /**
    Remove all punctuation.
    */
    fn sanitize_line(&self, line: String) -> String {
        let mut line: String = line;
        line.make_ascii_lowercase();

        let line = regexes::PUNCT_RM_CONTRACTIONS.replace_all(&line, "");
        let line = regexes::PUNCT_AT_END.replace_all(&line, "");
        let line = regexes::PUNCT_RM_U85_BR.replace_all(&line, " ");
        let line = regexes::DOUBLE_WHITESPACE.replace_all(&line, " ");
        regexes::PUNCT_NOT_AT_END
            .replace_all(&line, " ")
            .into_owned()
    }

    /**
    Change the number of tokens to create. For this to take effect, you must call
    `create_tokens` again.
    */
    fn set_max_tokens(&mut self, max_tokens: i32) {
        if max_tokens > 0 {
            self.max_tokens = max_tokens;
        }
    }

    /**
    Retrieves the list of string tokens. The order is random.
    */
    fn get_tokens(&self) -> Vec<String> {
        self.tokens.keys().map(|x| x.to_string()).collect()
    }

    /**
    Gets the term frequency of a token in the corpus.
    */
    fn term_frequency(&self, token: &str) -> u32 {
        self.tokens.get(token).expect("Token not found.").1
    }
}

#[cfg(test)]
mod tests {
    use crate::preprocessing::text::tokenizers::Tokenize;

    use super::*;

    #[test]
    fn create_tokens_test() {
        let mut st = SimpleTokenizer::new(100, true);
        st.create_tokens(&vec![
            String::from("Hello, my name is bob!"),
            String::from("Beep boop I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ]);
        let mut tokens = st.get_tokens();
        tokens.sort_unstable();
        let mut test_data = vec![
            "UNK", "beep", "bob", "a", "my", "im", "boop", "hello", "name", "is", "bot",
        ];
        test_data.sort_unstable();
        println!("{:?}", st.tokens);
        assert_eq!(tokens, test_data);
    }

    #[test]
    fn encode_test() {
        let mut st = SimpleTokenizer::new(100, true);
        st.create_tokens(&vec![
            String::from("Hello, my name is bob!"),
            String::from("Beep boop I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ]);
        let mut tokens = st.get_tokens();
        tokens.sort_unstable();
        let test_data: Vec<String> = vec![String::from("Hello, I'm Bloop!")];
        let test_data = st.encode(&test_data[0]);
        println!("{:?}", tokens);
        assert_eq!(test_data, Some(vec![1, 8, 0]));
    }

    #[test]
    fn decode_test() {
        let mut st = SimpleTokenizer::new(100, true);
        st.create_tokens(&vec![
            String::from("Hello, my name is bob!"),
            String::from("Beep boop I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ]);
        let mut tokens = st.get_tokens();
        tokens.sort_unstable();
        let test_data: Vec<String> = vec![String::from("Hello, I'm Bloop!")];
        let test_data = st.encode(&test_data[0]);

        assert_eq!(
            st.decode(&test_data.unwrap()).unwrap(),
            String::from("hello im UNK")
        )
    }
}
