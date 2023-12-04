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

    let mut point_total = 0;
    // Iterate through each line
    for (_line_number, line) in reader.lines().enumerate() {
        let line = line?; // If line is error, halt program

        let winning_nums: HashSet<&str> = line[COLON_INDEX + 1..PIPE_INDEX]
            .trim()
            .split_whitespace()
            .collect();

        let match_nums: Vec<&str> = line[PIPE_INDEX + 1..].trim().split_whitespace().collect();

        let card_value: u32 = process_line(winning_nums, match_nums);

        point_total += card_value;
    }

    println!("Part One Points: {}", point_total);
    Ok(())
}

fn process_line(set: HashSet<&str>, matches: Vec<&str>) -> u32 {
    let mut points: u32 = 0;
    let mut scoring: bool = false;

    for num in matches {
        if set.contains(&num) {
            if !scoring {
                scoring = true;
                points = 1;
            } else {
                points = points * 2;
            }
        }
    }

    points
}
