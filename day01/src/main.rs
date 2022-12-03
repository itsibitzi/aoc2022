use std::fs;

fn main() {
    let mut current_total = 0;

    let mut calories = vec![];

    fs::read_to_string("input")
        .unwrap()
        .lines()
        .for_each(|line| {
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
    let top_three = calories[0] + calories[1] + calories[2];
    println!("{}", top_three);
}
