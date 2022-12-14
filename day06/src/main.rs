use common::{elapsed, start_timer};
use day06::{part_1_simd, part_1_unrolled};

const FILE_BYTES: &[u8] = include_bytes!("../input.txt");

fn main() {
    let start_1 = start_timer();
    println!(
        "Part 1: {} in {}",
        part_1_unrolled(&FILE_BYTES),
        elapsed(&start_1)
    );
}
