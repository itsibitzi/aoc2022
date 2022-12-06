#![feature(portable_simd)]

use std::simd::{u8x16, Mask, Simd, SimdPartialEq};

const O: bool = true;
const X: bool = false;
const PART_1_GATHER_LHS_IDX: [usize; 16] = [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3];
const PART_1_GATHER_RHS_IDX: [usize; 16] = [0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
const PART_1_MASK: [bool; 16] /* pad! */ = [X, O, O, O, O, X, O, O, O, O, X, O, O, O, O, X];

pub fn part_1_simd(bytes: &[u8]) -> usize {
    let lhs_gatherer = Simd::from_array(PART_1_GATHER_LHS_IDX);
    let rhs_gatherer = Simd::from_array(PART_1_GATHER_RHS_IDX);
    let mask = Mask::from_array(PART_1_MASK);
    let mask_i8 = Mask::<i8, 16>::from_array(PART_1_MASK);
    let or = Simd::from_array([0; 16]);

    bytes
        .windows(4)
        .position(|window_bytes| unsafe {
            let lhs = u8x16::gather_select_unchecked(window_bytes, mask, lhs_gatherer, or);
            let rhs = u8x16::gather_select_unchecked(window_bytes, mask, rhs_gatherer, or);

            let check = mask_i8 & lhs.simd_eq(rhs);

            !check.any()
        })
        .unwrap()
        + 4
}

// Copied from Mario Savarese, only here so I can run benchmarks.
pub fn part_1_basic(bytes: &[u8], packet_size: usize) -> usize {
    bytes
        .windows(packet_size)
        .position(|window| !(1..packet_size).any(|i| window[i..].contains(&window[i - 1])))
        .unwrap()
        + packet_size
}
