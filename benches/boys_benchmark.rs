use boys::{exact as boys_exact, micb25 as boys_micb25};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("boys_exact_2", |b| b.iter(|| boys_exact::boys(2, 2.0)));
    c.bench_function("boys_exact_14", |b| {
        b.iter(|| boys_exact::boys(14, 42.67768466983068))
    });
    c.bench_function("boys_micb25_2", |b| b.iter(|| boys_micb25::boys(2, 2.0)));
    c.bench_function("boys_micb25_14", |b| {
        b.iter(|| boys_micb25::boys(14, 42.67768466983068))
    });
    // TODO these are disabled until their correctness can be determined.
    // c.bench_function("boys1_jeffhammond_2", |b| {
    //     b.iter(|| boys::jeffhammond::boys1(2, 2.0))
    // });
    // c.bench_function("boys1_jeffhammond_14", |b| {
    //     b.iter(|| boys::jeffhammond::boys1(14, 42.67768466983068))
    // });
    // c.bench_function("boys2_jeffhammond_2", |b| {
    //     b.iter(|| boys::jeffhammond::boys2(2, 2.0))
    // });
    // c.bench_function("boys2_jeffhammond_14", |b| {
    //     b.iter(|| boys::jeffhammond::boys2(14, 42.67768466983068))
    // });
    // c.bench_function("boys3_jeffhammond_2", |b| {
    //     b.iter(|| boys::jeffhammond::boys3(2, 2.0))
    // });
    // c.bench_function("boys3_jeffhammond_14", |b| {
    //     b.iter(|| boys::jeffhammond::boys3(14, 42.67768466983068))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
