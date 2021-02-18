use chess::Generator;
use chess::{Benchmark, Board, BoardGenerator};
use criterion::BenchmarkId;
use criterion::Throughput;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn par_test(c: &mut Criterion) {
    let sizes = (1..10).map(|i| i * 32);

    let mut group = c.benchmark_group("Rook captures");

    sizes.for_each(|size| {
        let generator = BoardGenerator::new(size);
        let board = generator.generate();

        group.bench_with_input(
            BenchmarkId::new("Single sequential", size),
            &size,
            |b, &size| {
                b.iter(|| board.get_rook_captures());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Multiple sequential", size),
            &size,
            |b, &size| {
                b.iter(|| board.get_rooks_captures());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Single parallel", size),
            &size,
            |b, &size| {
                b.iter(|| board.get_rook_captures_par());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Multiple parallel", size),
            &size,
            |b, &size| {
                b.iter(|| board.get_rooks_captures_par());
            },
        );
    });

    group.finish();
}

fn single(c: &mut Criterion) {
    let sizes = (1..10).map(|i| i * 32);

    let mut group = c.benchmark_group("Single only");

    sizes.for_each(|size| {
        let generator = BoardGenerator::new(size);

        group.bench_with_input(BenchmarkId::new("Sequential", size), &size, |b, &size| {
            b.iter(|| generator.generate().get_rook_captures());
        });

        group.bench_with_input(BenchmarkId::new("Parallel", size), &size, |b, &size| {
            b.iter(|| generator.generate().get_rook_captures_par());
        });
    });

    group.finish();
}

criterion_group!(benches, single);
criterion_main!(benches);
