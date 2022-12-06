use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day06::{part_1_basic, part_1_simd, part_1_unrolled};

const FILE_BYTES: &[u8] = include_bytes!("../input.txt");

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("basic", |b| {
        b.iter(|| part_1_basic(black_box(FILE_BYTES), 4))
    });
    c.bench_function("simd", |b| b.iter(|| part_1_simd(black_box(FILE_BYTES))));
    c.bench_function("unrolled", |b| {
        b.iter(|| part_1_unrolled(black_box(FILE_BYTES)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
