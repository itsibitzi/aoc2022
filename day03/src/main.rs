use std::fs;

use common::{elapsed, start_timer};
use itertools::Itertools;

#[inline]
fn score_item(item: u8) -> u8 {
    if item >= b'a' {
        item - 96
    } else {
        item - 64 + 26
    }
}

const ASCII_MAX: usize = 0b01111111;

fn part_1(file_text: &str) -> u32 {
    // Use a fixed sized buffer to store if we've already checked a given item
    let mut already_checked = [false; ASCII_MAX];

    file_text.lines().fold(0, |acc, line| {
        let line = line.as_bytes();
        let total = line.len();
        let half_total = total >> 1;

        let compartment_1 = &line[..half_total];
        let compartment_2 = &line[half_total..];

        let mut local_acc: u32 = 0;
        for item_1 in compartment_1 {
            if already_checked[*item_1 as usize] == false {
                already_checked[*item_1 as usize] = true;

                for item_2 in compartment_2 {
                    if item_1 == item_2 {
                        let score = score_item(*item_1) as u32;
                        local_acc += score;
                        break;
                    }
                }
            }
        }

        already_checked = [false; ASCII_MAX];
        acc + local_acc
    })
}

fn part_2(file_text: &str) -> u32 {
    file_text
        .lines()
        .tuples::<(_, _, _)>()
        .fold(0, |acc, (bag_1, bag_2, bag_3)| {
            let bag_1 = bag_1.as_bytes();
            let bag_2 = bag_2.as_bytes();
            let bag_3 = bag_3.as_bytes();

            for item_1 in bag_1 {
                if bag_2.contains(item_1) && bag_3.contains(item_1) {
                    return acc + score_item(*item_1) as u32;
                }
            }

            panic!("We couldn't find a badge in this group");
        })
}

fn main() {
    let file_text = fs::read_to_string("day03/input.txt").unwrap();

    let start_1 = start_timer();
    println!("Part 1: {} in {}", part_1(&file_text), elapsed(&start_1));

    let start_2 = start_timer();
    println!("Part 2: {} in {}", part_2(&file_text), elapsed(&start_2));
}
