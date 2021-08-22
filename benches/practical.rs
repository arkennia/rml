use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use rustml::knn;
use rustml::math;
use std::error::Error;

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

pub fn pratical_benchmark(c: &mut Criterion) {
    let training_data = parse_csv(TRAIN_FILE_NAME).unwrap();
    let testing_data = parse_csv(TEST_FILE_NAME).unwrap();
    let ind: usize = rand::thread_rng().gen_range(0..testing_data.0.len());

    let knn = knn::KNN::new(
        5,
        training_data.0,
        training_data.1,
        None,
        Some(math::norm::Norm::L2),
    );
    c.bench_function("Pratical", |b| {
        b.iter(|| knn.predict(black_box(&testing_data.0[ind])));
    });
}

criterion_group!(benches, pratical_benchmark);
criterion_main!(benches);
