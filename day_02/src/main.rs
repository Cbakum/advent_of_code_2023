use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Create hashmap
    let map = create_map(); // For part one

    // open file to search through data
    let filename: String = "data.txt".to_string();
    let file: File = File::open(filename)?; // If open is error, halt program
    let reader = io::BufReader::new(file);

    let mut valid_game_count: u32 = 0; // to track part one
    let mut power_total = 0; // to track part two

    // Iterate through each line
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?; // If line is error, halt program

        // for part one
        if parse_line_part_one(&line, &map) {
            let line_num: u32 = line_number as u32 + 1;
            valid_game_count += line_num;
        }

        // for part two
        power_total += parse_line_part_two(&line);
    }

    println!("Number of valid games {}", valid_game_count);
    println!("Power: {}", power_total);
    Ok(())
}

fn parse_line_part_two(line: &str) -> u32 {
    let mut quantity: u32 = 0;
    let mut max_cubes = initialize_color_max_values(); // Track max value for cubes

    // Split line into words
    for item in line.split_whitespace() {
        // Make string from slice to remove punctuation
        let word: String = item
            .to_string()
            .chars()
            .filter(|&c| c != ',' && c != ';')
            .collect();

        // check if current word is number or color
        match word.parse::<u32>() {
            Ok(parsed_value) => {
                quantity = parsed_value;
            }
            Err(_) => {
                // Use max for case of "Game" in input
                if max_cubes.get(&word).unwrap_or(&u32::MAX) < &quantity {
                    max_cubes.insert(word, quantity);
                }
            }
        }
    }

    // Get updated values
    let red_max = max_cubes.get("red").unwrap();
    let blue_max = max_cubes.get("blue").unwrap();
    let green_max = max_cubes.get("green").unwrap();

    // calculate product
    red_max * blue_max * green_max
}

fn parse_line_part_one(line: &str, map: &HashMap<String, u32>) -> bool {
    //let mut color: String = "".to_string();
    let mut quantity: u32 = 0;
    let mut valid: bool = true;

    for item in line.split_whitespace() {
        let word: String = item
            .to_string()
            .chars()
            .filter(|&c| c != ',' && c != ';')
            .collect();

        // check if current word is number or color
        match word.parse::<u32>() {
            Ok(parsed_value) => {
                quantity = parsed_value;
            }
            Err(_) => {
                //color = word;
                match map.get(&word) {
                    Some(&limit) => {
                        if quantity > limit {
                            valid = false;
                        }
                    }
                    None => {} // Do nothing
                }
            }
        }
    }
    valid
}

/// This map contains the limit for each color in task one
fn create_map() -> HashMap<String, u32> {
    let mut map = HashMap::new();
    map.insert("blue".to_string(), 14);
    map.insert("green".to_string(), 13);
    map.insert("red".to_string(), 12);
    map
}

/// This map will track the running maximum quantity for each color in task two
fn initialize_color_max_values() -> HashMap<String, u32> {
    let mut map = HashMap::new();
    map.insert("blue".to_string(), 0);
    map.insert("green".to_string(), 0);
    map.insert("red".to_string(), 0);
    map
}
