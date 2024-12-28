#![feature(portable_simd)]
#![feature(iter_array_chunks)]
#![allow(dead_code, unused_variables)]

pub mod faster_solver;
pub mod naive_solver;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution {
    pub a: u64,
    pub b: u64,
}
