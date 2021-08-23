// Copyright 2021 Jonathan Manly.

// This file is part of rustml.

// rustml is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Foobar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with Foobar.  If not, see <https://www.gnu.org/licenses/>.

use rustml::knn;
use rustml::math;
use std::error::Error;
use std::time::Instant;

const TRAIN_FILE_NAME: &str = "./data/optdigits.tra";
const TEST_FILE_NAME: &str = "./data/optdigits.tes";

type CSVOutput = (Vec<Vec<f64>>, Vec<i32>);

fn parse_csv(data: &str) -> Result<CSVOutput, Box<dyn Error>> {
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

fn main() -> Result<(), Box<dyn Error>> {
    // Format: (Vectors of each feature, Vector of class label)
    let training_data = parse_csv(TRAIN_FILE_NAME)?;
    let testing_data = parse_csv(TEST_FILE_NAME)?;

    let start = Instant::now();

    let knn = knn::KNN::new(
        5,
        training_data.0,
        training_data.1,
        None,
        Some(math::norm::Norm::L2),
    );

    // Find a better way to do this.
    let pred: Vec<i32> = testing_data.0.iter().map(|x| knn.predict(x)).collect();

    let num_correct = pred
        .iter()
        .cloned()
        .zip(&testing_data.1)
        .filter(|(a, b)| *a == **b)
        .count();

    println!(
        "Accuracy: {} Runtime: {}s",
        (num_correct as f64) / (pred.len() as f64),
        start.elapsed().as_secs_f64()
    );

    Ok(())
}
