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

use crate::preprocessing::text::tokenizers;
use regex::Regex;

#[derive(Debug, Clone, Default)]
pub struct SimpleTokenizer {
    pub data: Vec<String>,
    pub max_tokens: i32,
    tokens: Vec<String>,
}

impl SimpleTokenizer {
    pub fn new(data: Vec<String>, max_tokens: i32) -> Self {
        Self {
            data,
            max_tokens,
            tokens: Vec::new(),
        }
    }
}

impl tokenizers::Tokenize for SimpleTokenizer {
    fn create_tokens(&self) -> Vec<String> {
        let rm_punct = Regex::new("[,@#!\\?\"']").unwrap();
        let split_on_whitespace = Regex::new("[^A-Za-z0-9]").unwrap();
        let mut output: Vec<String> = Vec::new();
        let mut lower_buffer = String::new();
        for entry in &self.data {
            lower_buffer.push_str(entry);
            lower_buffer.make_ascii_lowercase();
            let entry = rm_punct.replace_all(&lower_buffer, ""); // FIXME: Improve this to avoid throwing this out repeatedly.
            output.extend(split_on_whitespace.split(&entry).map(String::from));
            lower_buffer.clear();
        }

        output
    }

    fn encode(&self, _input: &[String]) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn decode(&self, _input: &[i32]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::preprocessing::text::tokenizers::Tokenize;

    use super::*;

    #[test]
    fn create_tokens_test() {
        let st = SimpleTokenizer::new(
            vec![
                String::from("Hello, my name is bob!"),
                String::from("Beep boop I'm a bot"),
            ],
            100,
        );
        let tokens = st.create_tokens();
        println!("{:?}", tokens);
        assert!(false);
    }
}
