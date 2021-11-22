use criterion::{black_box, criterion_group, criterion_main, Criterion};
use semver_rs::Version;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Version 2.0.0", |b| {
        b.iter(|| black_box(Version::new("2.0.0").parse().ok()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
