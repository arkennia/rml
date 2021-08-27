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

pub mod simple_tokenizer;

pub trait Tokenize {
    fn create_tokens(&mut self, data: &Vec<String>);
    fn encode(&self, input: &String) -> Option<Vec<i32>>;
    fn decode(&self, input: &[i32]) -> Result<String, Box<dyn Error>>;
    fn set_max_tokens(&mut self, max_tokens: i32);
    fn get_tokens(&self) -> Vec<String>;
}
