use criterion::{black_box, criterion_group, criterion_main, Criterion};
use benchmark_ing::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    // c.bench_function("fib 10", |b| b.iter(|| fibonacci(black_box(10))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);