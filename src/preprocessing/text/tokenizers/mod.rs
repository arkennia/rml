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
Contains the tokenizers that are used by the vectorizers.

Defines the `Tokenize` trait for creating Tokenizers.
*/

pub mod bag_of_words;

pub use bag_of_words::BagOfWords;

use super::Ngrams;
pub trait Tokenize {
    fn create_tokens(&mut self, data: &[String]);
    fn encode(&self, input: &str) -> Option<Vec<i32>>;
    fn decode(&self, input: &[i32]) -> Option<String>;
    fn sanitize_line(&self, line: String) -> String;
    fn set_max_tokens(&mut self, max_tokens: i32);
    fn set_use_lowercase(&mut self, use_lowercase: bool);
    fn set_stop_words(&mut self, stop_words: Option<Vec<String>>);
    fn set_ngrams(&mut self, ngrams: Ngrams);
    fn get_tokens(&self) -> Vec<String>;
    fn get_term_frequency(&self, token: &str) -> u32;
    fn get_doc_count(&self) -> i32;
    fn get_token_from_idx(&self, idx: usize) -> String;
}
