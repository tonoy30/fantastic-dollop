use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sorting::{fibo, fibo_rec};

pub fn fibo_benchmark(c: &mut Criterion) {
    c.bench_function("fibo 2000", |b| b.iter(|| fibo(black_box(2000))));
}

pub fn fibo_recursive_benchmark(c: &mut Criterion) {
    c.bench_function("fibo recursive 2000", |b| {
        b.iter(|| fibo_rec(black_box(2000)))
    });
}

criterion_group!(benches, fibo_benchmark, fibo_recursive_benchmark);
criterion_main!(benches);
