#![no_std]
extern crate nalgebra as na;
use na::{ArrayStorage, Dynamic, SMatrix, U2, U3};

fn main() {
    let m = SMatrix::<u32, 3, 4>::new(11, 12, 13, 14, 21, 22, 23, 24, 31, 32, 33, 34);
}
