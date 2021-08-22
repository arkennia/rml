use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use std::collections::HashSet;
use std::iter::{self, FromIterator};

const DATASIZES: [usize; 6] = [100, 1000, 10000, 100000, 1000000, 10000000];

pub fn create_data(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let values = Vec::from_iter(iter::repeat_with(|| rng.gen_range(0..100)).take(size));
    values
}

pub fn get_num_labels(y: &[i32]) -> usize {
    let mut labels: Vec<i32> = Vec::new();

    for i in y {
        if !labels.contains(i) {
            labels.push(*i);
        }
    }

    labels.len()
}

pub fn get_num_labels_hashset(y: &[i32]) -> usize {
    let set: HashSet<i32> = HashSet::from_iter(y.iter().cloned());
    set.len()
}

pub fn get_num_labels_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get Num Labels");

    for i in DATASIZES {
        group.bench_with_input(BenchmarkId::new("Vector", i), &i, |b, i| {
            b.iter(|| get_num_labels(black_box(&create_data(*i))))
        });
        group.bench_with_input(BenchmarkId::new("Hashset", i), &i, |b, i| {
            b.iter(|| get_num_labels_hashset(black_box(&create_data(*i))))
        });
    }
}

criterion_group!(benches, get_num_labels_benchmark);
criterion_main!(benches);
