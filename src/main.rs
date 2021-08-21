use rustml::knn;
use rustml::math;
use std::error::Error;
use std::fs;
use std::process;

const TRAIN_FILE_NAME: &str = "./data/optdigits.tra";
const TEST_FILE_NAME: &str = "./data/optdigits.tes";

fn load_file(file_name: &str) -> String {
    let content = fs::read_to_string(file_name);

    if let Err(_) = content {
        panic!("Error opening file.");
    } else {
        content.unwrap()
    }
}

fn parse_csv(data: String) -> Result<(Vec<Vec<f64>>, Vec<i32>), Box<dyn Error>> {
    let mut out_data: (Vec<Vec<f64>>, Vec<i32>) = (Vec::new(), Vec::new());
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());

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

fn main() {
    // Format: (Vectors of each feature, Vector of class label)
    let training_data = parse_csv(load_file(TRAIN_FILE_NAME));
    let testing_data = parse_csv(load_file(TEST_FILE_NAME));

    if let Err(e) = training_data {
        println!("Application error: {}", e);
        process::exit(1);
    } else {
        let training_data = training_data.unwrap();
        let testing_data = testing_data.unwrap();
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

        let mut num_correct = 0;

        for i in 0..pred.len() {
            if pred[i] == testing_data.1[i] {
                num_correct += 1;
            }
        }

        println!("Accuracy: {}", (num_correct as f64) / (pred.len() as f64));
    }
}
