#![feature(portable_simd)]

use std::{
    fs,
    simd::{mask8x16, u8x16, u8x32, Mask, Simd, SimdPartialEq, SimdUint},
};

use common::{elapsed, start_timer};

// Annoyingly mask8x16::from_array is not const
const PART_1_MASK: [bool; 16] = [
    true, true, true, true, false, false, false, false, false, false, false, false, false, false,
    false, false,
];

const X: usize = 1337;
const PART_1_GATHER_LHS_IDX: [usize; 16] = [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3];
const PART_1_GATHER_RHS_IDX: [usize; 16] = [X, 0, 0, 0, 1, X, 1, 1, 2, 2, X, 2, 3, 3, 3, X];

fn part_1(bytes: &[u8]) -> usize {
    let lhs_gatherer = Simd::from_array(PART_1_GATHER_LHS_IDX);
    let rhs_gatherer = Simd::from_array(PART_1_GATHER_RHS_IDX);

    let mut idx = 0;

    loop {
        let window_bytes = &bytes[idx..idx + 4];

        let lhs = u8x16::gather_or_default(window_bytes, lhs_gatherer);
        let rhs = u8x16::gather_or_default(window_bytes, rhs_gatherer);

        let check = lhs.simd_ne(rhs);

        if check.all() {
            break;
        } else {
            idx += 1;
        }
    }

    idx + 4
}

fn main() {
    let file_text = fs::read_to_string("day06/input.txt").unwrap();
    let file_bytes = file_text.as_bytes();

    // let PART_2_MASK: mask8x16 = mask8x16::from_array([
    //     true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
    //     false,
    // ]);

    let start_1 = start_timer();
    println!("Part 1: {} in {}", part_1(&file_bytes), elapsed(&start_1));

    // let start_2 = start_timer();
    // println!("Part 2: {} in {}", part_2(&file_text), elapsed(&start_2));
}
