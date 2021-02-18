use chess::Generator;
use chess::{Benchmark, Board, BoardGenerator};
use criterion::BenchmarkId;
use criterion::Throughput;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn seq(c: &mut Criterion) {
    let sizes = (1..9).map(|i| i * 32);

    let mut group = c.benchmark_group("Sequential");

    sizes.for_each(|size| {
        let generator = BoardGenerator::new(size);
        group.bench_with_input(BenchmarkId::new("Single rook", size), &size, |b, &size| {
            b.iter(|| generator.generate().get_rook_captures());
        });

        group.bench_with_input(
            BenchmarkId::new("Multiple rooks", size),
            &size,
            |b, &size| {
                b.iter(|| generator.generate().get_rooks_captures());
            },
        );
    });

    group.finish();
}

criterion_group!(benches, seq);
criterion_main!(benches);
