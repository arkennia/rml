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
Frequency vectorizer module. Vectorizes text using the `max_features` most common tokens.

# Example
```rust
use rml::preprocessing::text;
let mut fv = text::FrequencyVectorizer::default();
fv.max_features = 50;

let data: Vec<String> = vec![String::from("Hello, world!")];

fv.gen_tokens(&data);
println!("{:?}", fv.vectorize(&data));
*/

use std::error::Error;

use crate::math::norm;
use crate::preprocessing::text::tokenizers;

use super::calculate_tfidf;
use super::frequencybuilder::FrequencyVectorizerBuilder;

/// The type of ngrams to keep.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ngrams {
    /// Single words only.
    Unigram,
    /// Dual words.
    Bigram,
    /// Both 1-gram and 2-grams.
    Both,
}

/**
The frequency vectorizer vectorizes text using the most common(highest frequency) tokens.
If you want to specify a different tokenizer besides `SimpleTokenizer` use the ::new method.
*/
pub struct FrequencyVectorizer {
    /// The number of tokens to keep. If this is changed you must call `gen_tokens` again. Set to -1 to keep all.
    pub max_features: i32,
    /// Make all tokens lowercase.
    use_lowercase: bool,
    /// Use TFIDF to encode characters.
    use_tfidf: bool,
    /// Optionally normalize the term frequency of each vector.
    norm: Option<norm::Norm>,
    /// Optionally remove the contained stop words.
    stop_words: Option<Vec<String>>,
    /// The type of ngrams. Unigrams means one word only.
    ngrams: Ngrams,
    /// The tokenizer to use contained in a Box.
    tokenizer: Box<dyn tokenizers::Tokenize>,
}

impl Default for FrequencyVectorizer {
    fn default() -> Self {
        Self {
            max_features: 10000,
            use_tfidf: false,
            use_lowercase: true,
            norm: None,
            stop_words: None,
            ngrams: Ngrams::Unigram,
            tokenizer: Box::new(tokenizers::BagOfWords::new(10000, true, None)),
        }
    }
}

impl FrequencyVectorizer {
    pub fn new(builder: FrequencyVectorizerBuilder) -> Self {
        Self {
            max_features: builder.max_features,
            use_tfidf: builder.use_tfidf,
            use_lowercase: builder.use_lowercase,
            norm: builder.norm,
            stop_words: builder.stop_words.clone(),
            ngrams: builder.ngrams,
            tokenizer: builder.tokenizer,
        }
    }

    /**
    Generate the tokens for use in vectorization. This step is required to use the vectorizer.
    */
    pub fn gen_tokens(&mut self, data: &[String]) {
        // Move stop words into the tokenizer.
        self.tokenizer.set_stop_words(self.stop_words.take());
        self.tokenizer.set_max_tokens(self.max_features);
        self.tokenizer.set_ngrams(self.ngrams);
        self.tokenizer.set_use_lowercase(self.use_lowercase);
        self.tokenizer.create_tokens(data);
    }

    /**
    Turns the given text into a vector utilizing the Bag of Words technique.
    The output is a vector containing each vector as `f64`.
     */
    pub fn vectorize(&self, input_data: &[String]) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
        let output: Vec<Vec<f64>> = input_data.iter().map(|x| self.vectorize_line(x)).collect();
        Ok(output)
    }

    /**
    Retrieves the tokens from the tokenizer. They are sorted according to index in the hashmap.
     */
    pub fn get_tokens(&self) -> Vec<String> {
        self.tokenizer.get_tokens()
    }

    fn vectorize_line(&self, line: &str) -> Vec<f64> {
        let i32_vec: Vec<i32> = self
            .tokenizer
            .encode(line)
            .expect("Error processing vector line.");
        let mut f64_vec: Vec<f64> = i32_vec.into_iter().map(|x| x as f64).collect();

        if let Some(norm) = self.norm {
            norm::normalize_vector(&mut f64_vec, &norm);
        }

        if self.use_tfidf {
            f64_vec = self.compute_vector_tfidf(f64_vec);
        }
        f64_vec
    }

    fn compute_vector_tfidf(&self, vector: Vec<f64>) -> Vec<f64> {
        let docs_in_corpus = self.tokenizer.get_doc_count();

        let mut out_vec: Vec<f64> = Vec::with_capacity(vector.len());
        out_vec.resize(vector.len(), 0.0);

        for (i, count) in vector.iter().enumerate() {
            let docs_with_token = self
                .tokenizer
                .get_term_frequency(&self.tokenizer.get_token_from_idx(i));
            out_vec[i] = calculate_tfidf(*count, docs_in_corpus, docs_with_token as i32);
        }

        out_vec
    }
}

#[cfg(test)]
mod tests {
    use super::FrequencyVectorizer;

    #[test]
    fn create_tokens_test() {
        let test_data = vec![
            String::from("Hello, my name is bob!"),
            String::from("Beep boop I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ];
        let mut vectorizer = FrequencyVectorizer::default();
        vectorizer.gen_tokens(&test_data);
        let test = vectorizer.vectorize(&vec![
            String::from("Hello, my name is bob!"),
            String::from("Beep boop I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ]);

        println!("{:?}", test);
        assert_eq!(
            test.unwrap(),
            vec![
                [0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0]
            ]
        );
    }
    #[test]
    fn create_tokens_with_tfidf_test() {
        let test_data = vec![
            String::from("Hello, my name is bob!"),
            String::from("Beep boop I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ];
        let mut vectorizer = FrequencyVectorizer::default();
        vectorizer.use_tfidf = true;
        vectorizer.gen_tokens(&test_data);
        let test = vectorizer.vectorize(&vec![
            String::from("Hello, my name is bob!"),
            String::from("Beep boop I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ]);

        println!("{:?}", test);
        assert_eq!(
            test.unwrap(),
            vec![
                [
                    0.0,
                    0.17609125905568124,
                    0.17609125905568124,
                    0.17609125905568124,
                    0.17609125905568124,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                ],
                [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.17609125905568124
                ],
                [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
            ]
        );
    }
}
