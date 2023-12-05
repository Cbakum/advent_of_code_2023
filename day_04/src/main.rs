use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // open file to search through data
    let filename: String = "data.txt".to_string();
    let file: File = File::open(filename)?; // If open is error, halt program
    let reader = io::BufReader::new(file);
    
    let deck_size = reader.lines().count(); // This consumes the reader

    let mut point_total = 0;
    let mut cards_total = 0;

    let mut card_copies: Vec<u32> = std::iter::repeat(1).take(deck_size).collect();

    // Reopen File
    let file: File = File::open("data.txt")?;
    let reader = io::BufReader::new(file);

    // Iterate through each line
    for (_line_number, line) in reader.lines().enumerate() {
        let line = line?; // If line is error, halt program

        let winning_nums: HashSet<u32> = line
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" | ")
            .nth(0)
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let match_nums: HashSet<u32> = line
            .split(" | ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

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

fn process_line(win_set: HashSet<u32>, match_set: HashSet<u32>) -> (u32, usize) {
    let mut points: u32 = 0;

    let intersections: HashSet<u32> = win_set.intersection(&match_set).cloned().collect();
    let num_matches = intersections.len();

    let wins = num_matches;
    if num_matches > 0 {
        points = 2u32.pow(num_matches as u32 - 1);
    }
    
    (points, wins)
}
