use std::time::Instant;

use vectorized_rust::*;

fn main() {
    let start = Instant::now();

    let solution = faster_solver_u64::solve::<4>(
        26,
        67,
        12748 + 10000000000000,
        66,
        21,
        12176 + 10000000000000,
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
