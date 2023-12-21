use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn renderling_benchmark(c: &mut Criterion) {
    c.bench_function("renderling_stub", |b| b.iter(|| black_box(1 + 1)));
}

criterion_group!(benches, renderling_benchmark);
criterion_main!(benches);
