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

let mut st = tokenizers::SimpleTokenizer::new(100, true, None);
st.create_tokens(&vec![
    String::from("Hello, my name is bob!"),
    String::from("Beep boop I'm a bot"),
    String::from("Beep boop I'm a bob!"),
]);
let mut t = st.get_tokens();
t.sort_unstable();
let mut test_data = vec!["UNK", "a", "beep", "bob", "boop", "bot", "hello", "i", "is", "m", "my", "name"];
test_data.sort_unstable();
assert_eq!(t, test_data);
```
*/

use crate::preprocessing::text::Ngrams;
use crate::preprocessing::text::regexes;
use crate::preprocessing::text::tokenizers;

use std::collections::HashMap;
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
    /// Total number of tokens to create. Set to -1 to keep all.
    pub max_tokens: i32,
    /// Make tokens lowercase.
    pub use_lowercase: bool,
    /// Stop words to remove.
    stop_words: Option<Vec<String>>,
    /// Type of ngrams to use.
    ngrams: Ngrams,
    /// The tokens generated and their index in the frequency vector.
    tokens: HashMap<String, (usize, u32)>,
}

impl Default for SimpleTokenizer {
    fn default() -> Self {
        Self {
            max_tokens: 10,
            use_lowercase: true,
            stop_words: None,
            ngrams: Ngrams::Unigram,
            tokens: Default::default(),
        }
    }
}

impl SimpleTokenizer {
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
            hashmap.insert(UNKNOWN_STR.to_string(), (UNKNOWN_IDX, 0));

            // Move hashmap to the tokenizer.
            self.tokens = hashmap;
        } else {
            hashmap.insert(UNKNOWN_STR.to_string(), (UNKNOWN_IDX, 0));

            // Move hashmap to the tokenizer.
            self.tokens = hashmap;
        }
    }

    fn create_ngrams(&self, line: Vec<String>) -> Vec<String> {
        match self.ngrams {
            Ngrams::Unigram => line,
            Ngrams::Bigram => SimpleTokenizer::compute_bigrams(&line),
            Ngrams::Both => SimpleTokenizer::compute_both_ngrams(line),
        }
    }

    pub fn compute_bigrams(line: &Vec<String>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for i in 0..line.len() - 1 {
            output.push(line[i].to_string() + " " + &line[i + 1].to_string());
        }
        output
    }

    #[inline]
    pub fn compute_both_ngrams(line: Vec<String>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        output.extend(SimpleTokenizer::compute_bigrams(&line));
        output.extend(line);
        output
    }

}

impl tokenizers::Tokenize for SimpleTokenizer {
    /**
        Create the tokens to use for tokenization of a text.
        It stores the created tokens internally, and can be retrieved with the `get_tokens` function.
    */
    // TODO: Parallelize
    fn create_tokens(&mut self, data: &[String]) {
        // <Token, (index, count)>
        let mut hashmap: HashMap<String, (usize, u32)> = HashMap::new();
        let mut line: Vec<String> = Vec::new();
        let mut doc_tokens: HashSet<String> = HashSet::new();

        for entry in data {
            let trimmed_entry = &entry.trim().to_string();
            // Extend the line vector with the strings contained in the split string.
            line.extend(
                self.sanitize_line(trimmed_entry.to_string())
                    .split(' ')
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            );
            line = self.create_ngrams(line);
            for x in &line {
                // If x is a stop word, ignore it.
                if let Some(stop_words) = &self.stop_words {
                    if stop_words.contains(&x) {
                        continue;
                    }
                }
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
            line.clear();
        }
        self.compute_most_frequent(hashmap);
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
                if let Some(stop_words) = &self.stop_words {
                    if stop_words.contains(&x.to_string()) {
                        continue;
                    }
                }
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

        let line = regexes::PUNCT_RM_CONTRACTIONS.replace_all(&line, " ");
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
    use crate::preprocessing::text;
    use crate::preprocessing::text::tokenizers::Tokenize;

    use super::*;

    #[test]
    fn create_tokens_test() {
        let mut st = SimpleTokenizer::new(100, true, Some(text::load_stop_words("english")));
        st.create_tokens(&vec![
            String::from("Hello, my name is bob!"),
            String::from("Beep boop I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ]);
        let mut tokens = st.get_tokens();
        tokens.sort_unstable();
        let mut test_data = vec!["UNK", "beep", "bob", "boop", "bot", "hello", "name"];
        test_data.sort_unstable();
        println!("{:?}", st.tokens);
        assert_eq!(tokens, test_data);
    }

    #[test]
    fn encode_test() {
        let mut st = SimpleTokenizer::new(100, true, None);
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
        assert_eq!(test_data, Some(vec![1, 8, 9, 0]));
    }

    #[test]
    fn decode_test() {
        let mut st = SimpleTokenizer::new(100, true, None);
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
            String::from("hello i m UNK")
        )
    }

    // We want to verify it is counting DOCUMENT frequency not CORPUS frequency.
    #[test]
    fn doc_count_test() {
        let mut st = SimpleTokenizer::new(100, true, None);
        st.create_tokens(&vec![
            String::from("Hello, my name is bob!"),
            String::from("Beep beep I'm a bot"),
            String::from("Beep boop I'm a bob!"),
        ]);

        assert_eq!(st.term_frequency("beep"), 2);
        assert_eq!(st.term_frequency("bob"), 2);
    }

    #[test]
    fn ngram_test() {
        let mut st = SimpleTokenizer::new(100, true, None);
        st.set_ngrams(Ngrams::Bigram);
        st.create_tokens(&vec![
            String::from("Hello, my name is bob!"),
        ]);
        let mut test_data = vec!["UNK", "hello my", "my name", "name is", "is bob"];
        test_data.sort_unstable();

        let mut st_tokens = st.get_tokens();
        st_tokens.sort_unstable();
                
        assert_eq!(st_tokens, test_data);
    }
}
