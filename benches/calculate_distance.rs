use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::Uniform;
use rand::Rng;
use rustml::knn;
use rustml::math::distance;
use rustml::math::norm;
use std::iter::{self, FromIterator};

const DIMENSIONS: usize = 8;
const DATASIZE: usize = 10000;

fn create_points(dims: usize, num_points: usize) -> Vec<Vec<f64>> {
    let mut data: Vec<Vec<f64>> = Vec::new();
    let between = Uniform::from(0.0..1.0);

    for _i in 0..num_points {
        let mut point: Vec<f64> = rand::thread_rng().sample_iter(between).take(dims).collect();
        norm::normalize_vector(&mut point, &norm::Norm::L2);
        data.push(point);
    }

    data
}

pub fn create_labels(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let values = Vec::from_iter(iter::repeat_with(|| rng.gen_range(0..10)).take(size));
    values
}

pub fn calculate_distances_benchmark(c: &mut Criterion) {
    let new_point = create_points(DIMENSIONS, 1);
    let knn = knn::KNN::new(
        5,
        create_points(DIMENSIONS, DATASIZE),
        create_labels(DATASIZE),
        Some(distance::Distance::Euclidean),
        Some(norm::Norm::L2),
    );
    c.bench_function("Calculate Distances", |b| {
        b.iter(|| knn.calculate_distances(black_box(&new_point[0])));
    });
}

criterion_group!(benches, calculate_distances_benchmark);
criterion_main!(benches);
