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
use csv::StringRecord;
use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

pub type CSVOutput<T, U> = (Vec<Vec<T>>, Vec<U>);

// pub fn parse_csv_with_labels<T, U>(
//     data: &str,
//     has_headers: bool,
// ) -> Result<CSVOutput<T, U>, Box<dyn Error>>
// where
//     T: FromStr + Debug,
//     T::Err: Debug,
//     U: FromStr + Debug,
//     U::Err: Debug,
// {
//     let mut out_data: CSVOutput<T, U> = (Vec::new(), Vec::new());
//     let mut reader = csv::ReaderBuilder::new()
//         .has_headers(has_headers)
//         .from_path(data)
//         .expect("Error creating CSV reader.");

//     for line in reader.records() {
//         let result = line?;
//         let mut line_data: (Vec<T>, U) = (
//             Vec::new(),
//             (result.get(result.len() - 1).unwrap())
//                 .trim()
//                 .parse()
//                 .expect("Error getting class label."),
//         );
//         line_data.1 = (result.get(result.len() - 1).unwrap())
//             .trim()
//             .parse()
//             .expect("Error getting class label.");
//         for i in 0..result.len() - 1 {
//             line_data.0.push(
//                 (result.get(i).unwrap())
//                     .trim()
//                     .parse()
//                     .expect("Error pushing data."),
//             );
//         }

//         out_data.0.push(line_data.0);
//         out_data.1.push(line_data.1);
//     }
//     Ok(out_data)
// }

pub fn parse_csv_with_labels<T, U>(
    data: &str,
    has_headers: bool,
) -> Result<CSVOutput<T, U>, Box<dyn Error>>
where
    T: FromStr + Debug,
    T::Err: Debug,
    U: FromStr + Debug,
    U::Err: Debug,
{
    let mut out_data: CSVOutput<T, U> = (Vec::new(), Vec::new());
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .from_path(data)
        .expect("Error creating CSV reader.");

    reader.records().into_iter().for_each(|x| {
        let mut line = x.unwrap();
        line.trim();
        out_data.1.push(process_label::<T, U>(&line));
        out_data.0.push(process_features::<T>(&line));
    });

    Ok(out_data)
}

fn process_features<T>(line: &StringRecord) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let mut features: Vec<T> = line.into_iter().map(|x| x.parse().unwrap()).collect();
    features.pop();
    features
}

fn process_label<T, U>(line: &StringRecord) -> U
where
    T: FromStr + Debug,
    T::Err: Debug,
    U: FromStr + Debug,
    U::Err: Debug,
{
    let label = (line.get(line.len() - 1).unwrap())
        .parse()
        .expect("Error getting class label.");
    label
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_csv_with_labels_test() {
        // Checks if strings load properly.
        let str_data: CSVOutput<String, i32> =
            parse_csv_with_labels("./data/test_data/str_test.csv", false)
                .expect("Error parsing csv.");

        assert_eq!(str_data.0.len(), 2);
        assert_eq!(str_data.1.len(), 2);

        assert_eq!(str_data.0[0][0], String::from("this is a string"));
        assert_eq!(str_data.1[0], 0);

        // Checks if floats load properly.
        let str_data: CSVOutput<f64, i32> =
            parse_csv_with_labels("./data/test_data/float_test.csv", false)
                .expect("Error parsing csv.");

        assert_eq!(str_data.0.len(), 2);
        assert_eq!(str_data.1.len(), 2);

        assert_eq!(str_data.0[1], vec![20.24, 3.823, 10.2]);
        assert_eq!(str_data.1[1], 1);
    }
}
