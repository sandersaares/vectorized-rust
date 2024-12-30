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
    // We will prepare vector versions of our constants
    // as vector operations require vectors for input.
    let x_a_vec = Simd::splat(x_a);
    let x_b_vec = Simd::splat(x_b);
    let x_vec = Simd::splat(x);
    let y_a_vec = Simd::splat(y_a);
    let y_b_vec = Simd::splat(y_b);
    let y_vec = Simd::splat(y);

    // This is a vector with the offsets used to go from a vector with the first
    // candidate A (splatted to all lanes) to all the candidates in a chunk.
    let mut candidate_offsets = Simd::splat(0);
    for lane_index in 0..CHUNK_SIZE {
        candidate_offsets[lane_index] = lane_index as u64;
    }

    // Identify max possible value of A, to stop loop if no solution found.
    let max_a = (x / x_a).min(y / y_a);

    // We iterate over candidate values of A in chunks of CHUNK_SIZE.
    // There is no guarantee that max A is a multiple of the chunk size!
    //
    // For optimal example realism here, we process "full" chunks with the vectorized
    // algorithm and fall back to the naive algorithm for processing any remainder.
    //
    // (We could simplify this code because we have an infinite mathematical sequence as
    // input but that is not realistic because real world inputs are typically finite.)
    let full_chunk_count = max_a / CHUNK_SIZE as u64;

    // Values for A that fall into the final partial chunk. May be an empty range.
    let partial_chunk_candidates = (CHUNK_SIZE as u64 * full_chunk_count)..=max_a;

    for chunk_index in 0..full_chunk_count {
        let first_candidate_in_chunk = chunk_index * CHUNK_SIZE as u64;

        // This gives us a vector with all the candidate A values in this chunk.
        // e.g. [0, 1, 2, 3] for the first chunk.
        let a_candidates =
            Simd::<_, CHUNK_SIZE>::splat(first_candidate_in_chunk) + candidate_offsets;

        let result = evaluate_chunk(
            a_candidates,
            x_a_vec,
            x_b_vec,
            x_vec,
            y_a_vec,
            y_b_vec,
            y_vec,
        );

        // We get booleans as output of the evaluation, answering "is this A a valid solution".
        // Notably, the evaluation result does not give the actual numeric value for B.
        // Therefore, if we found a valid solution, we still need to calculate B,
        // which we do by falling back to the naive algorithm.
        if result.any() {
            let solution_index = result
                .to_array()
                .iter()
                .position(|&x| x)
                .expect("we verified that at least one element is true");
            let a = a_candidates.as_array()[solution_index];

            return Some(
                evaluate_naive(a as u64, x_a, x_b, x, y_a, y_b, y)
                    .expect("we verified that this is a valid solution"),
            );
        }
    }

    // If there was any part of the sequence that didn't fit into a full chunk,
    // we process it with the naive algorithm.
    for a in partial_chunk_candidates {
        if let Some(solution) = evaluate_naive(a, x_a, x_b, x, y_a, y_b, y) {
            return Some(solution);
        }
    }

    None
}

// Evaluates whether a chunk of candidate values for A are valid solutions,
// with a nonzero value in the output vector indicating a valid solution.
fn evaluate_chunk<const CHUNK_SIZE: usize>(
    a_candidates: Simd<u64, CHUNK_SIZE>,
    x_a: Simd<u64, CHUNK_SIZE>,
    x_b: Simd<u64, CHUNK_SIZE>,
    x: Simd<u64, CHUNK_SIZE>,
    y_a: Simd<u64, CHUNK_SIZE>,
    y_b: Simd<u64, CHUNK_SIZE>,
    y: Simd<u64, CHUNK_SIZE>,
) -> Mask<i64, CHUNK_SIZE>
where
    LaneCount<CHUNK_SIZE>: SupportedLaneCount,
{
    // If we assume the given A, what is the expected value of Xb * B?
    let b_component_x = x - x_a * a_candidates;

    // Probe what a matching value for B might be for our current A.
    let b_x = b_component_x / x_b;

    // If not evenly divisible to yield a B, there is no solution with this A.
    let is_evenly_divisible_x = (b_x * x_b).simd_eq(b_component_x);

    // Do the same for Y.
    let b_component_y = y - y_a * a_candidates;
    let b_y = b_component_y / y_b;
    let is_evenly_divisible_y = (b_y * y_b).simd_eq(b_component_y);

    // Both the X and Y equations must yield the same value for B.
    let is_equal_x_y = b_x.simd_eq(b_y);

    // Combine all the conditions to yield an "is valid solution" boolean.
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
        let a_candidates = Simd::from_array([79, 80, 81, 82]);
        let x_a = Simd::splat(94);
        let x_b = Simd::splat(22);
        let x = Simd::splat(8400);
        let y_a = Simd::splat(34);
        let y_b = Simd::splat(67);
        let y = Simd::splat(5400);

        let result = evaluate_chunk(a_candidates, x_a, x_b, x, y_a, y_b, y);

        assert_eq!(result, Mask::from_array([false, true, false, false]));
    }
}
