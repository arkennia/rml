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

//! This benchmark compares the usage of iterators vs for loops. I was able to conclude that the iterator was slightly more performant.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use csv::StringRecord;
use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

pub type CSVOutput<T, U> = (Vec<Vec<T>>, Vec<U>);

pub fn parse_csv_with_labels_no_iter<T, U>(
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

    for line in reader.records() {
        let result = line?;
        let mut line_data: (Vec<T>, U) = (
            Vec::new(),
            (result.get(result.len() - 1).unwrap())
                .trim()
                .parse()
                .expect("Error getting class label."),
        );
        line_data.1 = (result.get(result.len() - 1).unwrap())
            .trim()
            .parse()
            .expect("Error getting class label.");
        for i in 0..result.len() - 1 {
            line_data.0.push(
                (result.get(i).unwrap())
                    .trim()
                    .parse()
                    .expect("Error pushing data."),
            );
        }

        out_data.0.push(line_data.0);
        out_data.1.push(line_data.1);
    }
    Ok(out_data)
}

pub fn parse_csv_with_labels_iter<T, U>(
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
        out_data.1.push(process_label::<T, U>(&x.as_ref().unwrap()));
        out_data.0.push(process_features::<T>(&x.unwrap()));
    });

    Ok(out_data)
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

fn process_features<T>(line: &StringRecord) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let mut features: Vec<T> = line.into_iter().map(|x| x.parse().unwrap()).collect();
    features.pop();
    features
}

pub fn parse_csv_with_labels_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parse CSV With Labels");

    // for i in DATASIZES {
    //     let data = create_data(i);
    //     group.bench_with_input(BenchmarkId::new("Vector", i), &i, |b, _i| {
    //         b.iter(|| get_num_labels(black_box(&data)))
    //     });
    //     group.bench_with_input(BenchmarkId::new("Hashset", i), &i, |b, _i| {
    //         b.iter(|| get_num_labels_hashset(black_box(&data)))
    //     });
    // }
    group.bench_function("Parse CSV with Labels No Iter", |b| {
        b.iter(|| {
            black_box(parse_csv_with_labels_no_iter::<f64, i32>(
                "./data/optdigits.tra",
                false,
            ))
        })
    });

    group.bench_function("Parse CSV with Labels Iter", |b| {
        b.iter(|| {
            black_box(parse_csv_with_labels_iter::<f64, i32>(
                "./data/optdigits.tra",
                false,
            ))
        })
    });
}

criterion_group!(benches, parse_csv_with_labels_benchmark);
criterion_main!(benches);
