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

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rml::preprocessing::text;

pub fn vectorizer_bench(c: &mut Criterion) {
    let mut fv = text::FrequencyVectorizer::default();
    let mut group = c.benchmark_group("IMDB Vectorization");
    group.sample_size(50);

    let data = text::csv::parse_csv_with_labels::<String, String>(
        "./data/test_data/IMDB_Dataset.csv",
        true,
        text::csv::ClassPosition::Last,
    )
    .unwrap();
    let data = text::flatten(data.0);
    fv.gen_tokens(&data);

    println!("{:?}", fv.get_tokens());

    group.bench_function("IMDB FreqVec w/ Simple Tokenizer", |b| {
        b.iter(|| fv.vectorize::<i32>(black_box(&data)))
    });
}

criterion_group!(benches, vectorizer_bench);
criterion_main!(benches);
