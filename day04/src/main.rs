use std::{fs::File, io::Read};

use common::{elapsed, start_timer};

trait Comparison {
    fn compare(buf: &[u8; 4]) -> bool;
}

struct Part1;
impl Comparison for Part1 {
    fn compare(buf: &[u8; 4]) -> bool {
        let a1 = buf[0];
        let a2 = buf[1];
        let b1 = buf[2];
        let b2 = buf[3];

        (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2)
    }
}

struct Part2;
impl Comparison for Part2 {
    fn compare(buf: &[u8; 4]) -> bool {
        let a1 = buf[0];
        let a2 = buf[1];
        let b1 = buf[2];
        let b2 = buf[3];

        a2 >= b1 && a1 <= b2
    }
}

fn part_1_faster<T: Comparison>(file_bytes: &[u8]) -> u32 {
    // none of the numbers in the real data are larger than (10^2 - 1) so a two slot parse buffer
    let mut ip_buf: [u8; 2] = [0; 2];
    let mut ipb_idx = 0;

    // Buffer for comparisons
    let mut c_buf: [u8; 4] = [0; 4];
    let mut c_idx = 0;

    let mut score = 0;

    for byte in file_bytes {
        if byte.is_ascii_digit() {
            ip_buf[ipb_idx] = *byte;
            ipb_idx += 1;
        } else {
            if ipb_idx == 1 {
                c_buf[c_idx] = ip_buf[0] - b'0';
            } else {
                c_buf[c_idx] = (ip_buf[0] - b'0') * 10 + (ip_buf[1] - b'0');
            }

            ipb_idx = 0;
            c_idx += 1;
        }

        if *byte == b'\n' {
            ipb_idx = 0;
            c_idx = 0;

            if T::compare(&c_buf) {
                score += 1;
            }
        }
    }

    if T::compare(&c_buf) {
        score += 1;
    }

    score
}

fn main() {
    let mut f = File::open("day04/input.txt").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();

    let start_1_f = start_timer();
    println!(
        "Part 1: {} in {}",
        part_1_faster::<Part1>(&buffer),
        elapsed(&start_1_f)
    );

    let start_2_f = start_timer();
    println!(
        "Part 2: {} in {}",
        part_1_faster::<Part2>(&buffer),
        elapsed(&start_2_f)
    );
}
