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

// You should have received a copy of the GNU Lesser General Public License
// along with rml.  If not, see <https://www.gnu.org/licenses/>.

//! Module for loading of built in stop words lists provided by NLTK.

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const STOP_WORD_LOCATION: &str = "./data/stopwords/";

pub fn load_stop_words(lang: &str) -> Vec<String> {
    let f = File::open(String::from(STOP_WORD_LOCATION) + &lang.to_ascii_lowercase())
        .expect("Couldn't find specified stop word lang.");
    let f = BufReader::new(f);

    f.lines()
        .into_iter()
        .map(|x| x.unwrap().trim().to_string())
        .collect::<Vec<String>>()
}
