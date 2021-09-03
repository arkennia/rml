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
use rml::preprocessing::text;

const DATASIZES: [i32; 3] = [50, 100, 1000];

pub fn tokenizer_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("IMDB Tokenization");
    group.sample_size(10);

    let data = text::csv::parse_csv_with_labels::<String, String>(
        "./data/test_data/IMDB_Dataset.csv",
        true,
        text::csv::ClassPosition::Last,
    )
    .unwrap();
    let data = text::flatten(data.0);

    for i in DATASIZES {
        let mut fv = text::FrequencyVectorizer::default();
        fv.max_features = i;
        group.throughput(Throughput::Elements(data.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("IMDB FreqVec w/ Simple Tokenizer", i),
            &i,
            |b, _| b.iter(|| fv.gen_tokens(black_box(&data))),
        );
    }
}

criterion_group!(benches, tokenizer_bench);
criterion_main!(benches);
