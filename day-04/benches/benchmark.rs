use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day_04::{number_to_digits_string, number_to_digits};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("number_to_digits", |b| b.iter(|| number_to_digits(black_box(452_123_845_366_159_001))));
    c.bench_function("number_to_digits_string", |b| b.iter(|| number_to_digits_string(black_box(452_123_845_366_159_001))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);