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
Provides functionality for tokenizing and vectorizing data retrieved by the `csv` module.
*/

pub mod frequency;
pub mod frequencybuilder;
pub mod hashing;

pub use frequency::*;
pub use frequencybuilder::FrequencyVectorizerBuilder;
pub use hashing::*;

pub fn calculate_tfidf(count_in_doc: f64, docs_in_corpus: i32, docs_containing_token: i32) -> f64 {
    if count_in_doc == 0.0 {
        return 0.0;
    }

    let tf = 1.0 + (count_in_doc).log10();
    let idf = (docs_in_corpus as f64 / (1.0 + docs_containing_token as f64)).log10();
    tf * idf
}

#[cfg(test)]
mod tests {
    use crate::preprocessing::text::calculate_tfidf;

    #[test]
    fn calculate_tfidf_test() {
        let tmp = calculate_tfidf(3.0, 50000, 1000);
        println!("{:?}", tmp);
        assert!((2.5089435194649936 - calculate_tfidf(3.0, 50000, 1000)).abs() < f64::EPSILON);
    }
}
