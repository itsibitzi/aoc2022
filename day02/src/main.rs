use std::fs;

const LOSS: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

#[allow(dead_code)]
const PART_1_SCORE_MATRIX: [[u32; 3]; 3] = [
    // A - rock
    [
        ROCK + DRAW, // X - rock
        ROCK + LOSS, // Y - paper
        ROCK + WIN,  // Z - scissors
    ],
    // B - paper
    [
        PAPER + WIN,  // X - rock
        PAPER + DRAW, // Y - paper
        PAPER + LOSS, // Z - scissors
    ],
    // C - scissors
    [
        SCISSORS + LOSS, // X - rock
        SCISSORS + WIN,  // Y - paper
        SCISSORS + DRAW, // Z - scissors
    ],
];

const PART_2_SCORE_MATRIX: [[u32; 3]; 3] = [
    // A - rock
    [
        SCISSORS + LOSS, // X - loss (scissors)
        ROCK + DRAW,     // Y - draw (rock)
        PAPER + WIN,     // Z - win (paper)
    ],
    // B - paper
    [
        ROCK + LOSS,    // X - loss (rock)
        PAPER + DRAW,   // Y - draw (paper)
        SCISSORS + WIN, // Z - win (scissors)
    ],
    // C - scissors
    [
        PAPER + LOSS,    // X - loss (paper)
        SCISSORS + DRAW, // Y - draw (scissors)
        ROCK + WIN,      // Z - win (rock)
    ],
];

fn main() {
    let file_text = fs::read_to_string("input.txt").unwrap();

    let score: u32 = file_text.lines().fold(0, |acc, line| {
        let opponent_move = line.as_bytes()[0];
        let your_move = line.as_bytes()[2];

        let opponent_index = opponent_move - b'A';
        let your_index = your_move - b'X';

        let row = PART_2_SCORE_MATRIX[opponent_index as usize];
        acc + row[your_index as usize]
    });

    println!("{score}");
}
