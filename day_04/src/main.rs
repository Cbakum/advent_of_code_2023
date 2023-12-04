use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // open file to search through data
    let filename: String = "data.txt".to_string();
    let file: File = File::open(filename)?; // If open is error, halt program
    let reader = io::BufReader::new(file);

    const COLON_INDEX: usize = 8;
    const PIPE_INDEX: usize = 40;
    const DECK_SIZE: usize = 190;

    let mut point_total = 0;
    let mut cards_total = 0;

    let mut card_copies: Vec<u32> = std::iter::repeat(1).take(DECK_SIZE).collect();

    // Iterate through each line
    for (_line_number, line) in reader.lines().enumerate() {
        let line = line?; // If line is error, halt program

        let winning_nums: HashSet<&str> = line[COLON_INDEX + 1..PIPE_INDEX]
            .trim()
            .split_whitespace()
            .collect();
        let match_nums: Vec<&str> = line[PIPE_INDEX + 1..].trim().split_whitespace().collect();

        let (card_value, wins) = process_line(winning_nums, match_nums);

        // Remove number of cards for current line from vector
        let multiplier: u32 = card_copies.remove(0);

        for i in 0..wins {
            // Increase number of cards down the line
            card_copies[i] += multiplier;
        }

        point_total += card_value; // For part one
        cards_total += multiplier; // For part two
    }

    println!(
        "Part One Points: {}       Part Two Total: {}",
        point_total, cards_total
    );
    Ok(())
}

fn process_line(set: HashSet<&str>, matches: Vec<&str>) -> (u32, usize) {
    let mut points: u32 = 0; // For part one
    let mut scoring: bool = false;
    let mut wins: usize = 0; // For part two

    for num in matches {
        if set.contains(&num) {
            if !scoring {
                scoring = true;
                points = 1;
                wins = 1;
            } else {
                points = points * 2;
                wins += 1;
            }
        }
    }
    (points, wins)
}