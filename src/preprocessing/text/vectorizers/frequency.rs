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

use std::error::Error;

use crate::math::norm;
use crate::preprocessing::text::tokenizers;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ngrams {
    Unigram,
    Bigram,
    Both,
}

pub struct FrequencyVectorizer {
    pub max_features: usize,
    pub features: Vec<String>,
    pub user_lowercase: bool,
    pub use_tfidf: bool,
    pub norm: Option<norm::Norm>,
    pub stop_words: Option<Vec<String>>,
    pub ngrams: Ngrams,
    tokenizer: Box<dyn tokenizers::Tokenize>,
}

impl Default for FrequencyVectorizer {
    fn default() -> Self {
        Self {
            max_features: 10000,
            features: Vec::new(),
            user_lowercase: true,
            use_tfidf: false,
            norm: None,
            stop_words: None,
            ngrams: Ngrams::Unigram,
            tokenizer: Box::new(tokenizers::simple_tokenizer::SimpleTokenizer::default()),
        }
    }
}

impl FrequencyVectorizer {
    pub fn new(
        max_features: usize,
        features: Vec<String>,
        user_lowercase: bool,
        use_tfidf: bool,
        norm: Option<norm::Norm>,
        stop_words: Option<Vec<String>>,
        ngrams: Ngrams,
        tokenizer: impl tokenizers::Tokenize + 'static,
    ) -> Self {
        Self {
            max_features,
            features,
            user_lowercase,
            use_tfidf,
            norm,
            stop_words,
            ngrams,
            tokenizer: Box::new(tokenizer),
        }
    }

    pub fn gen_tokens(&mut self) {
        self.features = self.tokenizer.create_tokens();
    }

    pub fn vectorize<T: From<i32>>(
        &self,
        input_data: &[Vec<String>],
    ) -> Result<Vec<Vec<T>>, Box<dyn Error>> {
        let output: Vec<Vec<T>> = input_data
            .iter()
            .map(|x| FrequencyVectorizer::vectorize_line(&*self.tokenizer, x))
            .collect();
        Ok(output)
    }

    fn vectorize_line<T: From<i32>>(
        tokenizer: &(impl tokenizers::Tokenize + ?Sized),
        line: &[String],
    ) -> Vec<T> {
        let i32_vec: Vec<i32> = tokenizer
            .encode(line)
            .expect("Error processing vector line.");
        i32_vec.iter().map(|x| T::from(*x)).collect()
    }
}
