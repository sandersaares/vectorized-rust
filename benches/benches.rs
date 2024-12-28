use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use vectorized_rust::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("naive", |b| {
        b.iter(|| {
            _ = black_box(naive_solver::solve(94, 22, 8400, 34, 67, 5400));
        });
    });

    c.bench_function("faster_1", |b| {
        b.iter(|| {
            _ = black_box(faster_solver::solve::<2>(94, 22, 8400, 34, 67, 5400));
        });
    });

    c.bench_function("faster_2", |b| {
        b.iter(|| {
            _ = black_box(faster_solver::solve::<2>(94, 22, 8400, 34, 67, 5400));
        });
    });

    c.bench_function("faster_4", |b| {
        b.iter(|| {
            _ = black_box(faster_solver::solve::<4>(94, 22, 8400, 34, 67, 5400));
        });
    });

    c.bench_function("faster_8", |b| {
        b.iter(|| {
            _ = black_box(faster_solver::solve::<8>(94, 22, 8400, 34, 67, 5400));
        });
    });

    c.bench_function("faster_16", |b| {
        b.iter(|| {
            _ = black_box(faster_solver::solve::<16>(94, 22, 8400, 34, 67, 5400));
        });
    });

    // The mere existence of these will slow down the above variations (code layout or something?).
    // c.bench_function("faster_32", |b| {
    //     b.iter(|| {
    //         _ = black_box(faster_solver::solve::<32>(94, 22, 8400, 34, 67, 5400));
    //     });
    // });

    // c.bench_function("faster_64", |b| {
    //     b.iter(|| {
    //         _ = black_box(faster_solver::solve::<64>(94, 22, 8400, 34, 67, 5400));
    //     });
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
