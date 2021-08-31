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
Contains lazy static versions of the regexes used so they don't need to be recompiled
during every function call.
*/

use regex::Regex;

lazy_static! {
    /// Finds all punctuation and symbols within a string. Safe to replace with "".
    pub static ref PUNCT_AT_END: Regex = Regex::new("[^\\w\\s]\\B").unwrap();
    /// Finds all punctuation not at the end(not followed by a space). We want to replace with " " when using this one.
    pub static ref PUNCT_NOT_AT_END: Regex = Regex::new("[^\\w\\s']\\b").unwrap();
    pub static ref PUNCT_RM_CONTRACTIONS: Regex = Regex::new("[']\\b").unwrap();
    pub static ref PUNCT_RM_U85_BR: Regex = Regex::new("(\\u{85})|(<br )").unwrap();
    pub static ref DOUBLE_WHITESPACE: Regex = Regex::new("\\s{2,}").unwrap();
    /// Finds everything not a character or number. Used to split on whitespace.
    pub static ref FIND_WHITESPACE: Regex = Regex::new("[^A-Za-z0-9]").unwrap();
    // Punct at end follow by a space. Safe to just remove and replace with "".
    //pub static ref PUNCT_FOLLOWED_BY_SPACE_AT_END: Regex = Regex::new("(([,@#!\\?\"'.])\\B)(?= )").unwrap();
}
