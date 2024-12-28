use num::Integer;

use crate::Solution;

pub fn solve(x_a: u64, x_b: u64, x: u64, y_a: u64, y_b: u64, y: u64) -> Option<Solution> {
    // Identify max possible value of A, to stop loop if no solution found.
    let max_a = (x / x_a).min(y / y_a);

    for a in 0..=max_a {
        if let Some(solution) = evaluate_naive(a, x_a, x_b, x, y_a, y_b, y) {
            return Some(solution);
        }
    }

    None
}

pub(crate) fn evaluate_naive(
    a_candidate: u64,
    x_a: u64,
    x_b: u64,
    x: u64,
    y_a: u64,
    y_b: u64,
    y: u64,
) -> Option<Solution> {
    // If we have this A, what is the expected value of Xb * B?
    let remaining_x = x - x_a * a_candidate;
    let remaining_y = y - y_a * a_candidate;

    let (b_x, remainder_x) = remaining_x.div_rem(&x_b);

    // If not evenly divisible, there is no solution with this A.
    if remainder_x != 0 {
        return None;
    }

    let (b_y, remainder_y) = remaining_y.div_rem(&y_b);

    // If not evenly divisible, there is no solution with this A.
    if remainder_y != 0 {
        return None;
    }

    // If value for B does not match, there is no solution with this A.
    if b_x != b_y {
        return None;
    }

    Some(Solution {
        a: a_candidate,
        b: b_x,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            solve(94, 22, 8400, 34, 67, 5400),
            Some(Solution { a: 80, b: 40 })
        );
        assert_eq!(solve(26, 67, 12748, 66, 21, 12176), None);
        assert_eq!(
            solve(17, 84, 7870, 86, 37, 6450),
            Some(Solution { a: 38, b: 86 })
        );
        assert_eq!(solve(69, 27, 18641, 23, 71, 10279), None);
    }
}
