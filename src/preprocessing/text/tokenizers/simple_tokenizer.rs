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
punctuation, and then splits the rest into word chunks.

# Example
```rust
use rml::preprocessing::text::tokenizers;
use rml::preprocessing::text::tokenizers::Tokenize;
let mut st = tokenizers::SimpleTokenizer::new(100);
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

use std::collections::HashSet;

// The unknown token string.
const UNKNOWN_STR: &str = "UNK";
// The index of the unknown token.
const UNKNOWN_IDX: usize = 0;

/**
Contains the data and options for the tokenizer.
*/
#[derive(Debug, Clone)]
pub struct SimpleTokenizer {
    /// Total number of tokens to create.
    pub max_tokens: usize,
    /// The tokens we generate.
    tokens: Vec<String>,
}

impl Default for SimpleTokenizer {
    fn default() -> Self {
        Self {
            max_tokens: 10,
            tokens: Default::default(),
        }
    }
}

impl SimpleTokenizer {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            max_tokens,
            tokens: Vec::new(),
        }
    }
}

impl tokenizers::Tokenize for SimpleTokenizer {
    /**
        Create the tokens to use for tokenization of a text.
        It stores the created tokens internally, and can be retrieved wit the `get_tokens` function.
    */
    // TODO: Implement feature mapping to only keep the x most popular. (hashmap)
    // TODO: Add more regexes for handling more punctuation (IE not at the beginning of a sentence with no proper spacing)
    // TODO: End of sentenence punctuation.
    fn create_tokens(&mut self, data: &[String]) {
        let mut tokens: Vec<String> = vec![UNKNOWN_STR.to_string()];

        let mut hashset: HashSet<String> = HashSet::new();
        for entry in data {
            for x in regexes::FIND_WHITESPACE.split(&self.sanitize_line(entry.to_string()).trim()) {
                hashset.insert(x.to_string());
            }
        }
        let mut tmp = hashset
            .into_iter()
            .take(self.max_tokens as usize)
            .collect::<Vec<String>>();
        tmp.sort_unstable();
        tokens.extend(tmp);
        self.tokens = tokens;
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
            for x in regexes::FIND_WHITESPACE.split(&self.sanitize_line(input.trim().to_string())) {
                output.push(
                    self.tokens
                        .binary_search(&x.to_string())
                        .unwrap_or(UNKNOWN_IDX) as i32,
                );
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
                output.push_str(&self.tokens[*word as usize]);
                output.push(' ');
            }
            Some(output.trim().to_string())
        } else {
            None
        }
    }

    /**
    Change the number of tokens to create. For this to take effect, you must call
    `create_tokens` again.
    */
    fn set_max_tokens(&mut self, max_tokens: usize) {
        if max_tokens > 0 {
            self.max_tokens = max_tokens;
        }
    }

    fn get_tokens(&self) -> Vec<String> {
        self.tokens.clone()
    }

    fn sanitize_line(&self, line: String) -> String {
        let mut line: String = line;
        line.make_ascii_lowercase();
        let buffer = regexes::PUNCT_RM_CONTRACTIONS.replace_all(&line, "");
        let buffer = regexes::PUNCT_AT_END.replace_all(&buffer, "");
        regexes::PUNCT_NOT_AT_END
            .replace_all(&buffer, " ")
            .into_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::preprocessing::text::tokenizers::Tokenize;

    use super::*;

    #[test]
    fn create_tokens_test() {
        let mut st = SimpleTokenizer::new(100);
        st.create_tokens(&vec![
            String::from("Hello, my name is bob!"),
            String::from("Beep boop I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ]);
        st.tokens.sort_unstable();
        let mut test_data = vec![
            "UNK", "beep", "bob", "a", "my", "im", "boop", "hello", "name", "is", "bot",
        ];
        test_data.sort_unstable();
        assert_eq!(st.tokens, test_data);
    }

    #[test]
    fn encode_test() {
        let mut st = SimpleTokenizer::new(100);
        st.create_tokens(&vec![
            String::from("Hello, my name is bob!"),
            String::from("Beep boop I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ]);
        st.tokens.sort_unstable();
        let test_data: Vec<String> = vec![String::from("Hello, I'm Bloop!")];
        let test_data = st.encode(&test_data[0]);
        assert_eq!(test_data, Some(vec![6, 7, 0]));
    }

    #[test]
    fn decode_test() {
        let mut st = SimpleTokenizer::new(100);
        st.create_tokens(&vec![
            String::from("Hello, my name is bob!"),
            String::from("Beep boop I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ]);
        st.tokens.sort_unstable();
        let test_data: Vec<String> = vec![String::from("Hello, I'm Bloop!")];
        let test_data = st.encode(&test_data[0]);

        assert_eq!(
            st.decode(&test_data.unwrap()).unwrap(),
            String::from("hello im UNK")
        )
    }
}
