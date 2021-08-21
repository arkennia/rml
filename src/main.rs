use rustml::knn;
use rustml::math;
use std::error::Error;

const TRAIN_FILE_NAME: &str = "./data/optdigits.tra";
const TEST_FILE_NAME: &str = "./data/optdigits.tes";

type CSVOutput = (Vec<Vec<f64>>, Vec<i32>);

fn parse_csv(data: &str) -> Result<CSVOutput, Box<dyn Error>> {
    let mut out_data: (Vec<Vec<f64>>, Vec<i32>) = (Vec::new(), Vec::new());
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

    let knn = knn::KNN::new(
        5,
        training_data.0,
        training_data.1,
        None,
        Some(math::norm::Norm::L2),
    );

    // Find a better way to do this.
    let pred: Vec<i32> = testing_data
        .0
        .iter()
        .map(|x| knn.predict(x.to_vec()))
        .collect();

    // let mut num_correct = 0;

    // for i in 0..pred.len() {
    //     if pred[i] == testing_data.1[i] {
    //         num_correct += 1;
    //     }
    // }
    let num_correct = pred
        .iter()
        .cloned()
        .zip(&testing_data.1)
        .filter(|(a, b)| *a == **b)
        .count();

    println!("Accuracy: {}", (num_correct as f64) / (pred.len() as f64));

    Ok(())
}
