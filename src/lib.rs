#![feature(portable_simd)]

pub mod faster_solver;
pub mod faster_solver_u64;
pub mod naive_solver;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution {
    pub a: u64,
    pub b: u64,
}
