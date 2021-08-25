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
use std::error::Error;

pub type CSVOutput = (Vec<Vec<f64>>, Vec<i32>);

pub fn parse_csv(data: &str) -> Result<CSVOutput, Box<dyn Error>> {
    let mut out_data: CSVOutput = (Vec::new(), Vec::new());
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(data)?;

    for line in reader.records() {
        let result = line?;
        let mut line_data: (Vec<f64>, i32) = (Vec::new(), 0);
        line_data.1 = (result.get(result.len() - 1).unwrap()).parse()?;
        for i in 0..result.len() - 1 {
            line_data.0.push((result.get(i).unwrap()).parse()?);
        }

        out_data.0.push(line_data.0);
        out_data.1.push(line_data.1);
    }
    Ok(out_data)
}
