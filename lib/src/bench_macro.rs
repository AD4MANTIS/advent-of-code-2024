#[macro_export]
macro_rules! bench_part {
    ($part: expr) => {
        use criterion::{criterion_group, criterion_main, Criterion};

        fn criterion_benchmark(c: &mut Criterion) {
            c.bench_function(stringify!($part), |b| {
                b.iter(|| $part());
            });
        }

        criterion::criterion_group!(benches, criterion_benchmark);
        criterion::criterion_main!(benches);
    };
}
