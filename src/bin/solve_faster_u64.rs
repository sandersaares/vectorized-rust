use std::{hint::black_box, time::Instant};

use vectorized_rust::*;

fn main() {
    let start = Instant::now();

    // We use black_box() to prevent the compiler from performing optimizations
    // that depend on the specific values we provide here. This is because we are
    // interested in the behavior of this algorithm in the general case.
    let solution = faster_solver_u64::solve::<4>(
        black_box(26),
        black_box(67),
        black_box(12748 + 10000000000000),
        black_box(66),
        black_box(21),
        black_box(12176 + 10000000000000),
    );
    assert_eq!(
        solution,
        Some(Solution {
            a: 118679050709,
            b: 103199174542
        })
    );

    let duration = start.elapsed();

    println!("Time elapsed: {} milliseconds", duration.as_millis());
}
