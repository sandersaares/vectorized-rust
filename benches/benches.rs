use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use vectorized_rust::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    const X_A: u64 = 94;
    const X_B: u64 = 22;
    const X: u64 = 8400 + 94 * 123456;
    const Y_A: u64 = 34;
    const Y_B: u64 = 67;
    const Y: u64 = 5400 + 34 * 123456;

    c.bench_function("naive", |b| {
        b.iter(|| {
            _ = black_box(naive_solver::solve(
                black_box(X_A),
                black_box(X_B),
                black_box(X),
                black_box(Y_A),
                black_box(Y_B),
                black_box(Y),
            ));
        });
    });

    c.bench_function("faster_1", |b| {
        b.iter(|| {
            _ = black_box(faster_solver::solve::<1>(
                black_box(X_A),
                black_box(X_B),
                black_box(X),
                black_box(Y_A),
                black_box(Y_B),
                black_box(Y),
            ));
        });
    });

    c.bench_function("faster_2", |b| {
        b.iter(|| {
            _ = black_box(faster_solver::solve::<2>(
                black_box(X_A),
                black_box(X_B),
                black_box(X),
                black_box(Y_A),
                black_box(Y_B),
                black_box(Y),
            ));
        });
    });

    c.bench_function("faster_4", |b| {
        b.iter(|| {
            _ = black_box(faster_solver::solve::<4>(
                black_box(X_A),
                black_box(X_B),
                black_box(X),
                black_box(Y_A),
                black_box(Y_B),
                black_box(Y),
            ));
        });
    });

    c.bench_function("faster_4_u64", |b| {
        b.iter(|| {
            _ = black_box(faster_solver_u64::solve::<4>(
                black_box(X_A),
                black_box(X_B),
                black_box(X),
                black_box(Y_A),
                black_box(Y_B),
                black_box(Y),
            ));
        });
    });

    c.bench_function("faster_8", |b| {
        b.iter(|| {
            _ = black_box(faster_solver::solve::<8>(
                black_box(X_A),
                black_box(X_B),
                black_box(X),
                black_box(Y_A),
                black_box(Y_B),
                black_box(Y),
            ));
        });
    });

    c.bench_function("faster_16", |b| {
        b.iter(|| {
            _ = black_box(faster_solver::solve::<16>(
                black_box(X_A),
                black_box(X_B),
                black_box(X),
                black_box(Y_A),
                black_box(Y_B),
                black_box(Y),
            ));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
