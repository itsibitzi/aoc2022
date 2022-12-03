use std::fs;

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

    // We could store the top 3 in some values here
    // which would mean we don't have to do the sort/reverse
    // at the end but I'm too lazy to rework this.
    let mut calories = vec![];

    file_text.lines().for_each(|line| {
        if line == "\n" || line == "" {
            calories.push(current_total);
            current_total = 0;
        } else {
            let value: u32 = line.parse().unwrap();
            current_total += value;
        }
    });

    calories.sort();
    calories.reverse();
    calories[0] + calories[1] + calories[2]
}

fn main() {
    let file_text = fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part_1(&file_text));
    println!("Part 2: {}", part_2(&file_text));
}
