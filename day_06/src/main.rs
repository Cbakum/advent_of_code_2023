use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {

    /************************************************************/
    //                          PART ONE                        //
    /************************************************************/

    let filename: String = "data.txt".to_string();
    let file: File = File::open(&filename)?; // If open is error, halt program
    let reader = io::BufReader::new(file);

    let mut race_times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    let mut lines = reader.lines();

    if let Some(first_line) = lines.next() {
        race_times = first_line
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
    }

    if let Some(second_line) = lines.next() {
        distances = second_line
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
    }

    let num_races: usize = distances.len();
    let mut winning_charge_times: Vec<u32> = Vec::new();

    // Brute force it
    for race in 0..num_races {
        let mut win_count: u32 = 0;
        for charge_time in 0..race_times[race] {
            if try_race(race_times[race], charge_time) > distances[race] {
                win_count += 1;
            }
        }

        winning_charge_times.push(win_count);
    }

    let mut product = 1;
    for item in winning_charge_times {
        product *= item;
    }

    println!("Part one answer: {}", product);


    /************************************************************/
    //                          PART TWO                        //
    /************************************************************/

    let file: File = File::open(filename)?; // If open is error, halt program
    let reader = io::BufReader::new(file);

    let mut race_time: u64 = 0;
    let mut distance: u64 = 0;

    let mut lines = reader.lines();

    if let Some(first_line) = lines.next() {
        race_time = first_line
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>() // Use collect::<String>() to accumulate into a String
            .parse()
            .unwrap();
    }
    if let Some(second_line) = lines.next() {
        distance = second_line
            .unwrap()
            .trim()
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>() // Use collect::<String>() to accumulate into a String
            .parse()
            .unwrap();
    }

    let mut win_count: u32 = 0;
    for charge_time in 0..race_time {
        if try_race(race_time, charge_time) > distance {
            win_count += 1;
        }
    }

    println!("Part two answer: {}", win_count);

    Ok(())
}

fn try_race(duration: u64, charge_time: u64) -> u64 {
    let mut distance: u64 = 0;
    
    if charge_time < duration {
        let run_time = duration - charge_time;
        distance = charge_time * run_time;
    }

    distance
}