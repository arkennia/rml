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
Builds a new FrequencyVectorizer.
All options are configurable at run time and will
use sane defaults.
*/

use super::frequency::*;
use crate::preprocessing::text::vectorizers::Ngrams;
use crate::{math::norm, preprocessing::text::tokenizers};

/**
    A builder for creating FreqeuncyVectorizers.
*/
pub struct FrequencyVectorizerBuilder {
    /// The number of features to keep. Set to -1 to keep all.
    pub max_features: i32,
    /// Make all tokens lowercase.
    pub use_lowercase: bool,
    /// Use TFIDF to encode characters.
    pub use_tfidf: bool,
    /// Optionally normalize each vector.
    pub norm: Option<norm::Norm>,
    /// Optionally remove the contained stop words.
    pub stop_words: Option<Vec<String>>,
    /// The type of ngrams. Unigrams means one word only.
    pub ngrams: Ngrams,
    /// The tokenizer to use contained in a Box.
    pub tokenizer: Box<dyn tokenizers::Tokenize>,
}

impl FrequencyVectorizerBuilder {
    pub fn new(max_features: i32, use_lowercase: bool) -> Self {
        Self {
            max_features,
            use_lowercase,
            use_tfidf: false,
            norm: None,
            stop_words: None,
            ngrams: Ngrams::Unigram,
            tokenizer: Box::new(tokenizers::BagOfWords::new(
                max_features,
                use_lowercase,
                None,
            )),
        }
    }

    /// Toggle Term-Frequency Inverse Document Frequency on or off.
    pub fn with_tfidf(mut self, tfidf: bool) -> Self {
        self.use_tfidf = tfidf;
        self
    }

    /// Set L1 or L2 normalization. This will default to none if not set with this function.
    pub fn with_norm(mut self, norm: norm::Norm) -> Self {
        self.norm = Some(norm);
        self
    }

    /// Use stop words when tokenizing. Will default to none.
    pub fn with_stop_words(mut self, stop_words: Vec<String>) -> Self {
        self.stop_words = Some(stop_words);
        self
    }

    /// Set the 1 or 2-ngram option, or both. Defaults to Unigrams.
    pub fn with_ngram_type(mut self, ngrams: Ngrams) -> Self {
        self.ngrams = ngrams;
        self
    }

    /// Set the tokenizer to use. Defaults to `SimpleTokenizer`.
    pub fn with_tokenizer(mut self, tokenizer: Box<dyn tokenizers::Tokenize>) -> Self {
        self.tokenizer = tokenizer;
        self
    }

    /// Consume this builder and create a `FrequencyVectorizer`.
    pub fn build(self) -> FrequencyVectorizer {
        FrequencyVectorizer::new(self)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn create_tokens_test() {
//         let test_data = vec![
//             String::from("Hello, my name is bob!"),
//             String::from("Beep boop I'm a bot"),
//             String::from("Beep boop I'm a bob!"),
//         ];
//         let builder = FrequencyVectorizerBuilder::new(12, true)
//             .with_ngram_type(Ngrams::Unigram)
//             .with_tfidf(false)
//             .with_tokenizer(Box::new(tokenizers::BagOfWords::new(10, true, None)))
//             .with_tfidf(true);
//         let mut vectorizer = builder.build();
//         vectorizer.gen_tokens(&test_data);
//         let test = vectorizer.vectorize(&vec![
//             String::from("Hello, my name is bob!"),
//             String::from("Beep boop I'm a bot"),
//             String::from("Beep boop I'm a bob!"),
//         ]);
//         println!("{:?}", vectorizer.get_tokens());
//         println!("{:?}", test);
//         assert_eq!(
//             test.unwrap(),
//             vec![
//                 [
//                     0.0,
//                     0.17609125905568124,
//                     0.17609125905568124,
//                     0.17609125905568124,
//                     0.17609125905568124,
//                     0.0,
//                     0.0,
//                     0.0,
//                     0.0,
//                     0.0,
//                     0.0,
//                     0.0
//                 ],
//                 [
//                     0.0,
//                     0.0,
//                     0.0,
//                     0.0,
//                     0.0,
//                     0.0,
//                     0.0,
//                     0.0,
//                     0.0,
//                     0.0,
//                     0.0,
//                     0.17609125905568124
//                 ],
//                 [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
//             ]
//         );
//     }
// }
