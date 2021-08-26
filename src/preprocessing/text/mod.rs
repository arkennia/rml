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

//! Module that allows for loading and parsing of data.

pub mod csv;
pub mod tokenizers;
pub mod vectorizers;

/**
Consumes the given vector `v` of type Vec<Vec<T>> and flattens it to Vec<T>.

Can be used to flatten the data pulled from the `csv` module. An example usage would be
where each line is a string. This will create a vector of strings instead of Vec<Vec<String>>.
*/
pub fn flatten<T>(v: Vec<Vec<T>>) -> Vec<T> {
    v.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_test() {
        let str_data: Vec<Vec<String>> =
            csv::parse_csv_without_labels("./data/test_data/one_line_strings.csv", false)
                .expect("Error parsing csv.");
        let str_data = flatten(str_data);
        assert_eq!(str_data[0], String::from("i am a string"));
    }
}
