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

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rml::math::norm;
use rml::preprocessing::text;
use rml::preprocessing::text::vectorizers::Ngrams;

const DATASIZES: [i32; 4] = [50, 100, 1000, 10000];

pub fn vectorizer_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("IMDB Vectorization");
    group.sample_size(50);

    let data = text::csv::parse_csv_with_labels::<String, String>(
        "./data/test_data/IMDB_Dataset.csv",
        true,
        text::csv::ClassPosition::Last,
    )
    .unwrap();
    let data = text::flatten(data.0);

    for i in DATASIZES {
        let mut fv = text::FrequencyVectorizerBuilder::new(i, true)
            .with_ngram_type(Ngrams::Both)
            .with_norm(norm::Norm::L2)
            .with_tfidf(true)
            .build();
        fv.gen_tokens(&data);
        println!("{:?}", fv.max_features);
        println!("{:?}", fv.get_tokens());
        group.throughput(Throughput::Elements(data.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("IMDB FreqVec w/ Simple Tokenizer", i),
            &i,
            |b, _| b.iter(|| fv.vectorize(black_box(&data))),
        );
    }
}

criterion_group!(benches, vectorizer_bench);
criterion_main!(benches);
