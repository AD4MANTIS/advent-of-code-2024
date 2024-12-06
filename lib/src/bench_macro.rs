#[macro_export]
macro_rules! bench_part {
    ($part: expr $(, $sample_size: literal)?) => {
        use criterion::Criterion;

        fn criterion_benchmark(c: &mut Criterion) {
            c.bench_function(stringify!($part), |b| {
                b.iter(|| $part());
            });
        }

        criterion::criterion_group!(
            name = benches;
            config = Criterion::default()
            $(.sample_size($sample_size))?;
            targets = criterion_benchmark
        );
        criterion::criterion_main!(benches);
    };
}
