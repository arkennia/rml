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
*/

use std::error::Error;

use crate::math::norm;
use crate::preprocessing::text::tokenizers;

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
    // /// Make all tokens lowercase.
    // use_lowercase: bool,
    /// Use TFIDF to encode characters.
    use_tfidf: bool,
    /// Optionally normalize each vector.
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
            norm: None,
            stop_words: None,
            ngrams: Ngrams::Unigram,
            tokenizer: Box::new(tokenizers::SimpleTokenizer::new(10000, true, None)),
        }
    }
}

impl FrequencyVectorizer {
    pub fn new(builder: FrequencyVectorizerBuilder) -> Self {
        Self {
            max_features: builder.max_features,
            use_tfidf: builder.use_tfidf,
            norm: builder.norm,
            stop_words: builder.stop_words.clone(),
            ngrams: builder.ngrams,
            tokenizer: builder.tokenizer,
        }
    }

    pub fn gen_tokens(&mut self, data: &[String]) {
        // Move stop words into the tokenizer.
        self.tokenizer.set_stop_words(self.stop_words.take());
        self.tokenizer.set_max_tokens(self.max_features);
        self.tokenizer.create_tokens(data);
    }

    pub fn vectorize<T: From<i32>>(
        &self,
        input_data: &[String],
    ) -> Result<Vec<Vec<T>>, Box<dyn Error>> {
        let output: Vec<Vec<T>> = input_data
            .iter()
            .map(|x| FrequencyVectorizer::vectorize_line(&*self.tokenizer, x))
            .collect();
        Ok(output)
    }

    pub fn get_tokens(&self) -> Vec<String> {
        self.tokenizer.get_tokens()
    }

    fn vectorize_line<T: From<i32>>(
        tokenizer: &(impl tokenizers::Tokenize + ?Sized),
        line: &str,
    ) -> Vec<T> {
        let i32_vec: Vec<i32> = tokenizer
            .encode(line)
            .expect("Error processing vector line.");
        i32_vec.iter().map(|x| T::from(*x)).collect()
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
        let test = vectorizer.vectorize::<i32>(&vec![
            String::from("Hello, my name is bob!"),
            String::from("Beep boop I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ]);

        println!("{:?}", test);
        assert_eq!(
            test.unwrap(),
            vec![[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [6, 7, 8, 9, 5]]
        );
    }
}
