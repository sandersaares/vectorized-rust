use std::simd::{cmp::SimdPartialEq, LaneCount, Mask, Simd, SupportedLaneCount};

use crate::{naive_solver::evaluate_naive, Solution};

pub fn solve<const CHUNK_SIZE: usize>(
    x_a: u64,
    x_b: u64,
    x: u64,
    y_a: u64,
    y_b: u64,
    y: u64,
) -> Option<Solution>
where
    LaneCount<CHUNK_SIZE>: SupportedLaneCount,
{
    // Create vectors for our constants.
    let x_a_vec = Simd::splat(x_a as f64);
    let x_b_vec = Simd::splat(x_b as f64);
    let x_vec = Simd::splat(x as f64);
    let y_a_vec = Simd::splat(y_a as f64);
    let y_b_vec = Simd::splat(y_b as f64);
    let y_vec = Simd::splat(y as f64);

    // Identify max possible value of A, to stop loop if no solution found.
    let max_a = (x / x_a).min(y / y_a);

    // We iterate over candidate values of A in chunks of CHUNK_SIZE.
    // There is no guarantee that max A is a multiple of the chunk size
    // so once we calculate a result we check whether it actually falls in bounds.
    for first_candidate_in_chunk in (0..=max_a).step_by(CHUNK_SIZE) {
        let mut a_candidates = Simd::<f64, CHUNK_SIZE>::splat(0.0);

        for lane_index in 0..CHUNK_SIZE {
            // Note that this can yield candidate values for A that are out of bounds (above max_a).
            // This is OK in this case because we have a purely mathematical infinite input
            // sequence. If operating on real data, one would typically round down to the nearest
            // chunk boundary here and process any remainder with a naive algorithm (i.e. using
            // array_chunks() + into_remainder() instead of step_by()).
            let candidate = first_candidate_in_chunk + lane_index as u64;
            a_candidates[lane_index] = candidate as f64;
        }

        let result = evaluate_chunk(
            a_candidates,
            x_a_vec,
            x_b_vec,
            x_vec,
            y_a_vec,
            y_b_vec,
            y_vec,
        );

        if result.any() {
            let a = a_candidates.as_array()[result
                .to_array()
                .iter()
                .position(|&x| x)
                .expect("we verified that at least one element is true")];

            if a as u64 > max_a {
                // We went out of bounds - this is not a valid solution.
                return None;
            }

            // Use the naive algorithm to find the exact solution.
            // The fast algorithm just gets us a boolean that our solution was found.
            return Some(
                evaluate_naive(a as u64, x_a, x_b, x, y_a, y_b, y)
                    .expect("we verified that this is a valid solution"),
            );
        }
    }

    None
}

// Evaluates whether a chunk of candidate values for A are valid solutions,
// with a nonzero value in the output vector indicating a valid solution.
fn evaluate_chunk<const CHUNK_SIZE: usize>(
    a_candidates: Simd<f64, CHUNK_SIZE>,
    x_a: Simd<f64, CHUNK_SIZE>,
    x_b: Simd<f64, CHUNK_SIZE>,
    x: Simd<f64, CHUNK_SIZE>,
    y_a: Simd<f64, CHUNK_SIZE>,
    y_b: Simd<f64, CHUNK_SIZE>,
    y: Simd<f64, CHUNK_SIZE>,
) -> Mask<i64, CHUNK_SIZE>
where
    LaneCount<CHUNK_SIZE>: SupportedLaneCount,
{
    // If we have this A, what is the expected value of Xb * B?
    let remaining_x = x - x_a * a_candidates;

    // Probe what a matching value for B might be for our current A.
    let b_x = remaining_x / x_b;

    // If not evenly divisible to yield a B, there is no solution with this A.
    let is_evenly_divisible_x = (b_x * x_b).simd_eq(remaining_x);

    // Do the same for Y.
    let remaining_y = y - y_a * a_candidates;
    let b_y = remaining_y / y_b;
    let is_evenly_divisible_y = (b_y * y_b).simd_eq(remaining_y);

    // Both the X and Y equations must yield the same value for B.
    let is_equal_x_y = b_x.simd_eq(b_y);

    // Combine all the conditions to yield a valid solution.
    is_evenly_divisible_x & is_evenly_divisible_y & is_equal_x_y
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CHUNK_SIZE: usize = 4;

    #[test]
    fn examples() {
        assert_eq!(
            solve::<TEST_CHUNK_SIZE>(94, 22, 8400, 34, 67, 5400),
            Some(Solution { a: 80, b: 40 })
        );
        assert_eq!(solve::<TEST_CHUNK_SIZE>(26, 67, 12748, 66, 21, 12176), None);
        assert_eq!(
            solve::<TEST_CHUNK_SIZE>(17, 84, 7870, 86, 37, 6450),
            Some(Solution { a: 38, b: 86 })
        );
        assert_eq!(solve::<TEST_CHUNK_SIZE>(69, 27, 18641, 23, 71, 10279), None);
    }

    #[test]
    fn smoke_test_evaluate_chunk() {
        let a_candidates = Simd::from_array([79.0, 80.0, 81.0, 82.0]);
        let x_a = Simd::splat(94.0);
        let x_b = Simd::splat(22.0);
        let x = Simd::splat(8400.0);
        let y_a = Simd::splat(34.0);
        let y_b = Simd::splat(67.0);
        let y = Simd::splat(5400.0);

        let result = evaluate_chunk(a_candidates, x_a, x_b, x, y_a, y_b, y);

        assert_eq!(result, Mask::from_array([false, true, false, false]));
    }
}
