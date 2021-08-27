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

use crate::preprocessing::text::regexes;
use crate::preprocessing::text::tokenizers;

use std::collections::HashSet;

const UNKNOWN_STR: &str = "UNK";
const UNKNOWN_IDX: usize = 0;

#[derive(Debug, Clone)]
pub struct SimpleTokenizer {
    pub max_tokens: i32,
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
    pub fn new(max_tokens: i32) -> Self {
        Self {
            max_tokens,
            tokens: Vec::new(),
        }
    }
}

impl tokenizers::Tokenize for SimpleTokenizer {
    fn create_tokens(&mut self, data: &Vec<String>) {
        let mut tokens: Vec<String> = Vec::new();
        tokens.push(UNKNOWN_STR.to_string());

        let mut hashset: HashSet<String> = HashSet::new();
        let mut lower_buffer = String::new();
        for entry in data {
            lower_buffer.push_str(entry);
            lower_buffer.make_ascii_lowercase();
            let entry = regexes::RM_PUNCT.replace_all(&lower_buffer, ""); // FIXME: Improve this to avoid throwing this out repeatedly.

            for x in regexes::FIND_WHITESPACE.split(&entry) {
                hashset.insert(x.to_string());
            }

            lower_buffer.clear();
        }
        let mut tmp = hashset
            .into_iter()
            .take(self.max_tokens as usize)
            .collect::<Vec<String>>();
        tmp.sort_unstable();
        tokens.extend(tmp);
        self.tokens = tokens;
    }

    fn encode(&self, input: &String) -> Option<Vec<i32>> {
        if !self.tokens.is_empty() {
            let mut input = input.to_owned();
            input.make_ascii_lowercase();
            let input = regexes::RM_PUNCT.replace_all(&input, "");
            // Some(
            //     regexes::FIND_WHITESPACE
            //         .split(&input)
            //         .map(|x| {
            //             println!("{:?}", x.to_string());
            //             self.tokens
            //                 .binary_search(&x.to_string())
            //                 .expect("Error processing key") as i32
            //         })
            //         .collect::<Vec<i32>>(),
            // )
            let mut output: Vec<i32> = Vec::default();
            println!("{:?}", self.tokens);
            for x in regexes::FIND_WHITESPACE.split(&input) {
                println!("{:?}", x.to_string());
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

    fn decode(&self, _input: &[i32]) -> Result<String, Box<dyn std::error::Error>> {
        todo!()
    }

    fn set_max_tokens(&mut self, max_tokens: i32) {
        if max_tokens > 0 {
            self.max_tokens = max_tokens;
        }
    }

    fn get_tokens(&self) -> Vec<String> {
        self.tokens.clone()
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
}
