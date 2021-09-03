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

let mut st = tokenizers::BagOfWords::new(100, true, None);
st.create_tokens(&vec![
    String::from("Hello, my name is bob!"),
    String::from("Beep boop I'm a bot"),
    String::from("Beep boop I'm a bob!"),
]);
let mut t = st.get_tokens();
t.sort_unstable();
let mut test_data = vec!["UNK", "a", "beep", "bob", "boop", "bot", "hello", "i'm", "is", "my", "name"];
test_data.sort_unstable();
assert_eq!(t, test_data);
```
*/

use rayon::iter::ParallelIterator;
use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator};

use crate::preprocessing::text::Ngrams;
use crate::preprocessing::text::{regexes, tokenizers};

use std::collections::{HashMap, HashSet};

// The unknown token string.
const UNKNOWN_STR: &str = "UNK";
// The index of the unknown token.
const UNKNOWN_IDX: usize = 0;

/**
Contains the data and options for the tokenizer.
*/
#[derive(Debug, Clone)]
pub struct BagOfWords {
    /// Total number of tokens to create. Set to -1 to keep all.
    pub max_tokens: i32,
    /// Make tokens lowercase.
    pub use_lowercase: bool,
    /// Stop words to remove.
    stop_words: Option<Vec<String>>,
    /// Type of ngrams to use.
    ngrams: Ngrams,
    /// Number of documents in corpus.
    num_documents: i32,
    /// The tokens generated and their index in the frequency vector.
    tokens: HashMap<String, (usize, u32)>,
    /// Used to lookup by index.
    tokens_as_idx: Vec<String>,
}

impl Default for BagOfWords {
    fn default() -> Self {
        Self {
            max_tokens: 10,
            use_lowercase: true,
            stop_words: None,
            ngrams: Ngrams::Unigram,
            num_documents: 0,
            tokens: Default::default(),
            tokens_as_idx: Default::default(),
        }
    }
}

impl BagOfWords {
    pub fn new(max_tokens: i32, use_lowercase: bool, stop_words: Option<Vec<String>>) -> Self {
        Self {
            max_tokens,
            use_lowercase,
            stop_words,
            ..Self::default()
        }
    }

    // Gets the most frequent terms in the corpus by document frequency.
    // Used in the create tokens function.
    fn compute_most_frequent(&mut self, mut hashmap: HashMap<String, (usize, u32)>) {
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
            hashmap = self.adjust_indexes(hashmap);
            hashmap.insert(UNKNOWN_STR.to_string(), (UNKNOWN_IDX, 0));

            // Move hashmap to the tokenizer.
            self.tokens = hashmap;
        } else {
            hashmap.insert(UNKNOWN_STR.to_string(), (UNKNOWN_IDX, 0));

            // Move hashmap to the tokenizer.
            self.tokens = hashmap;
        }
    }

    fn adjust_indexes(
        &self,
        mut hashmap: HashMap<String, (usize, u32)>,
    ) -> HashMap<String, (usize, u32)> {
        for (i, k) in hashmap.iter_mut().enumerate() {
            k.1 .0 = i + 1;
        }
        hashmap
    }

    /**
    Creates the ngrams while tokenizing. This also removes stop words because we do not want them
    in our ngrams.
    */
    fn create_ngrams(&self, mut line: Vec<String>) -> Vec<String> {
        if let Some(stop_words) = &self.stop_words {
            line = line
                .into_iter()
                .filter(|x| !stop_words.contains(x))
                .collect::<Vec<String>>();
        }

        match self.ngrams {
            Ngrams::Unigram => line,
            Ngrams::Bigram => BagOfWords::compute_bigrams(&line),
            Ngrams::Both => BagOfWords::compute_both_ngrams(line),
        }
    }

    fn compute_bigrams(line: &Vec<String>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for i in 0..line.len() - 1 {
            output.push(line[i].to_string() + " " + &line[i + 1].to_string());
        }
        output
    }

    #[inline]
    fn compute_both_ngrams(line: Vec<String>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        output.extend(BagOfWords::compute_bigrams(&line));
        output.extend(line);
        output
    }

    /**
    Remove all punctuation.
    */
    fn process_data<'a>(&self, mut data: Vec<String>) -> Vec<Vec<String>> {
        if self.use_lowercase {
            data.par_iter_mut().for_each(|x| {
                x.make_ascii_lowercase();
            });
        }

        for i in 0..data.len() {
            let line = &data[i];
            //let line = regexes::PUNCT_RM_CONTRACTIONS.replace_all(&line, " ");
            let line = regexes::PUNCT_AT_END.replace_all(&line, "");
            // let line = regexes::PUNCT_RM_U85_BR.replace_all(&line, " ");
            // let line = regexes::DOUBLE_WHITESPACE.replace_all(&line, " ");

            let line = regexes::PUNCT_NOT_AT_END.replace_all(&line, " ");
            data[i] = line.trim().to_string();
        }

        let data: Vec<Vec<String>> = data
            .into_par_iter()
            .map(|x| x.split(' ').into_iter().map(|x| x.to_string()).collect())
            .collect();
        data.into_par_iter()
            .map(|x| self.create_ngrams(x))
            .collect()
    }
}

impl tokenizers::Tokenize for BagOfWords {
    /**
        Create the tokens to use for tokenization of a text.
        It stores the created tokens internally, and can be retrieved with the `get_tokens` function.
    */
    // TODO: Parallelize
    fn create_tokens(&mut self, data: &Vec<String>) {
        // <Token, (index, count)>
        let mut hashmap: HashMap<String, (usize, u32)> = HashMap::new();
        let mut doc_tokens: HashSet<String> = HashSet::new();
        self.num_documents = data.len() as i32;

        let data = data.to_owned();
        let data = self.process_data(data);

        for entry in data {
            // let trimmed_entry = &entry.trim().to_string();
            // // Extend the line vector with the strings contained in the split string.
            // line.extend(
            //     self.sanitize_line(trimmed_entry.to_string())
            //         .split(' ')
            //         .into_iter()
            //         .map(|x| x.to_string())
            //         .collect::<Vec<String>>(),
            // );
            // line = self.create_ngrams(line);
            for x in entry {
                // If this token, x, has already been seen in this document, do not count it.
                // This is because we want document frequency, not overall corpus frequency.
                if !doc_tokens.contains(&*x) {
                    let tmp = hashmap.insert(x.to_string(), (hashmap.len() + 1, 1));
                    if let Some(y) = tmp {
                        hashmap.insert(x.to_string(), (y.0, y.1 + 1));
                    }
                    doc_tokens.insert(x.clone()); // I don't really know a better way to do this.
                }
            }
            doc_tokens.clear();
        }
        hashmap.remove(" ");
        self.compute_most_frequent(hashmap);
        self.tokens_as_idx = self.get_tokens();
        // println!("{:?}", self.tokens);
    }

    /**
    Turn the given string into a vector of integers of size `max_features.` It will place
    the frequency of any token `x` in the corresponding index.

    # Note
    If the `create_tokens` function was not called before this one, it will return none.
    */
    fn encode(&self, input: &Vec<String>) -> Option<Vec<Vec<i32>>> {
        if !self.tokens.is_empty() {
            // let input = input.to_owned();
            let mut output: Vec<Vec<i32>> = Vec::with_capacity(input.len());
            output.resize(input.len(), Vec::with_capacity(self.tokens.len()));
            let mut input = self.process_data(input.to_owned());
            for (i, x) in input.iter_mut().enumerate() {
                if let Some(stop_words) = &self.stop_words {
                    x.retain(|x| !stop_words.contains(x));
                }
                output[i].resize(self.tokens.len(), 0);
                x.iter().for_each(|x| {
                    output[i][self.tokens.get(x).unwrap_or(&(UNKNOWN_IDX, 0)).0] += 1
                });
            }
            Some(output)
        } else {
            None
        }
    }

    /**
    Turn the given integer slice into a string matching the corrrect features,
    or place an `UNK` token for unknowns.

    If the ngrams and/or stop words options are used, this will not output a correct
    string. Due to the tokenizer removing all punctuation, it will also not have any punctuation output.

    # Note
    If the `create_tokens` function was not called before this one, it will return none.
    */
    // fn decode(&self, input: &[i32]) -> Option<String> {
    //     if !self.tokens.is_empty() {
    //         let mut output = String::new();
    //         for word in input {
    //             output.push_str(
    //                 &self
    //                     .tokens
    //                     .iter()
    //                     .find_map(|(k, val)| {
    //                         if val.0 == *word as usize {
    //                             Some(k.to_string())
    //                         } else {
    //                             None
    //                         }
    //                     })
    //                     .unwrap_or_else(|| UNKNOWN_STR.to_string()),
    //             );
    //             output.push(' ');
    //         }
    //         Some(output.trim().to_string())
    //     } else {
    //         None
    //     }
    // }

    /**
    This tokenizer uses a "Bag of Words" encoding process, and such order is not maintained.
    Therefore, it is not possible to decode, and so this will always return none.
     */
    fn decode(&self, _input: &[i32]) -> Option<String> {
        None
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
    Sets whether or not to use lowercase.
     */
    fn set_use_lowercase(&mut self, use_lowercase: bool) {
        self.use_lowercase = use_lowercase;
    }

    /**
    Sets the stops words to use.
     */
    fn set_stop_words(&mut self, stop_words: Option<Vec<String>>) {
        self.stop_words = stop_words;
    }

    fn set_ngrams(&mut self, ngrams: Ngrams) {
        self.ngrams = ngrams;
    }

    /**
    Retrieves the list of string tokens. The order is random.
    */
    fn get_tokens(&self) -> Vec<String> {
        let mut unsorted: Vec<String> = self.tokens.keys().map(|x| x.to_string()).collect();
        unsorted.sort_unstable_by(|x, b| {
            self.tokens
                .get(x)
                .unwrap()
                .0
                .cmp(&self.tokens.get(b).unwrap().0)
        });
        unsorted
    }

    /**
    Gets the raw term frequency of a token in the corpus.
    */
    fn get_term_frequency(&self, token: &str) -> u32 {
        self.tokens.get(token).expect("Token not found.").1
    }

    /**
    Get the total number of documents in the corpus.
     */
    fn get_doc_count(&self) -> i32 {
        self.num_documents
    }

    /**
    Retrieve the token from a given index. A reverse lookup.
     */
    fn get_token_from_idx(&self, idx: usize) -> String {
        self.tokens_as_idx[idx].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::preprocessing::text;
    use crate::preprocessing::text::tokenizers::Tokenize;

    use super::*;

    #[test]
    fn create_tokens_test() {
        let mut st = BagOfWords::new(100, true, Some(text::load_stop_words("english")));
        st.create_tokens(&vec![
            String::from("Hello,  my name is bob!"),
            String::from("Beep boop I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ]);
        let tokens = st.get_tokens();
        let test_data = vec!["UNK", "hello", "name", "bob", "i'm", "beep", "boop", "bot"];
        println!("{:?}", tokens);
        let tmp = tokens
            .iter()
            .filter(|&x| !test_data.contains(&&x[..]))
            .collect::<Vec<&String>>()
            .len();
        assert_eq!(tmp, 0);
    }

    // #[test]
    // fn encode_test() {
    //     let mut st = BagOfWords::new(100, true, None);
    //     st.create_tokens(&vec![
    //         String::from("Hello, my name is bob!"),
    //         String::from("Beep boop I'm a bot"),
    //         String::from("Beep boop I'm a bob!"),
    //     ]);
    //     let mut tokens = st.get_tokens().clone();
    //     tokens.sort_unstable();
    //     let test_data: Vec<String> = vec![String::from("Hello, I'm Bloop!")];
    //     let test_data = st.encode(&vec![test_data[0].to_owned()]);
    //     assert_eq!(
    //         test_data,
    //         Some(vec![vec![1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0]])
    //     );
    // }

    // We want to verify it is counting DOCUMENT frequency not CORPUS frequency.
    #[test]
    fn doc_count_test() {
        let mut st = BagOfWords::new(100, true, None);
        st.create_tokens(&vec![
            String::from("Hello, my name is bob!"),
            String::from("Beep beep I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ]);

        assert_eq!(st.get_term_frequency("beep"), 2);
        assert_eq!(st.get_term_frequency("bob"), 2);
    }

    #[test]
    fn bigram_test() {
        let mut st = BagOfWords::new(100, true, None);
        st.set_ngrams(Ngrams::Bigram);
        st.create_tokens(&vec![String::from("Hello, my name is bob!")]);
        let mut test_data = vec!["UNK", "hello my", "my name", "name is", "is bob"];
        test_data.sort_unstable();

        let mut st_tokens = st.get_tokens();
        st_tokens.sort_unstable();

        assert_eq!(st_tokens, test_data);
    }

    #[test]
    fn bothgram_test() {
        let mut st = BagOfWords::new(100, true, None);
        st.set_ngrams(Ngrams::Both);
        st.create_tokens(&vec![String::from("Hello, my name is bob!")]);
        let mut test_data = vec![
            "UNK", "hello my", "my name", "name is", "is bob", "hello", "my", "name", "is", "bob",
        ];
        test_data.sort_unstable();

        let mut st_tokens = st.get_tokens();
        st_tokens.sort_unstable();

        assert_eq!(st_tokens, test_data);
    }

    #[test]
    fn bothgram_with_stop_test() {
        let mut st = BagOfWords::new(
            100,
            true,
            Some(text::stop_words::load_stop_words("english")),
        );
        st.set_ngrams(Ngrams::Both);
        st.create_tokens(&vec![String::from("Hello, my name is bob!")]);
        let mut test_data = vec!["UNK", "bob", "hello", "hello name", "name", "name bob"];
        test_data.sort_unstable();

        let mut st_tokens = st.get_tokens();
        st_tokens.sort_unstable();

        assert_eq!(st_tokens, test_data);
    }
}
