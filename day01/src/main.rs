use std::fs;

use common::{elapsed, start_timer};

fn part_1(file_text: &str) -> u32 {
    let mut current_total = 0;

    let mut max_calories = 0;

    file_text.lines().for_each(|line| {
        if line == "\n" || line == "" {
            if current_total > max_calories {
                max_calories = current_total;
            }
            current_total = 0;
        } else {
            let value: u32 = line.parse().unwrap();
            current_total += value;
        }
    });

    max_calories
}

fn part_2(file_text: &str) -> u32 {
    let mut current_total = 0;

    let mut calories_1 = 0;
    let mut calories_2 = 0;
    let mut calories_3 = 0;

    file_text.lines().for_each(|line| {
        if line == "\n" || line == "" {
            if current_total > calories_1 {
                calories_3 = calories_2;
                calories_2 = calories_1;
                calories_1 = current_total;
            } else if current_total > calories_2 {
                calories_3 = calories_2;
                calories_2 = current_total;
            } else if current_total > calories_3 {
                calories_3 = current_total;
            }
            current_total = 0;
        } else {
            let value: u32 = line.parse().unwrap();
            current_total += value;
        }
    });

    calories_1 + calories_2 + calories_3
}

fn main() {
    let file_text = fs::read_to_string("day01/input.txt").unwrap();

    let start_1 = start_timer();
    println!("Part 1: {} in {}", part_1(&file_text), elapsed(&start_1));

    let start_2 = start_timer();
    println!("Part 2: {} in {}", part_2(&file_text), elapsed(&start_2));
}
