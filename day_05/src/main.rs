use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {

    /************************************************************/
    //                          PART ONE                        //
    /************************************************************/

    // open file to search through data
    let filename: String = "data.txt".to_string();
    let file: File = File::open(&filename)?; // If open is error, halt program
    let reader = io::BufReader::new(file);

    let mut seed_collection: Vec<i64> = Vec::new(); // Part one
    let mut mappings: Vec<Mapping> = Vec::new();

    // Collect data line by line
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?; // If line is error, halt program

        if line_number != 0 {
            // If map label is hit, create a new map, push it onto the vector
            if line.chars().any(|c| c.is_alphabetic()) {
                let new_map = Mapping::new();
                mappings.push(new_map);
                //println!("Alphabetic");
            } else if line.chars().any(|c| c.is_numeric()) {
                let inputs: Vec<i64> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                // Load inputs as rule into map
                if let Some(last_map) = mappings.last_mut() {
                    last_map.load_rule(inputs[0], inputs[1], inputs[2]);
                } else {
                    panic!("Failed to get a mutable reference to the last element in mappings.");
                }
            } else {
                // Nothing to do for a blank line
            }
        } else {
            // Grabs the seeds from the first line
            seed_collection = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
        }
    }

    let mut target_locations: Vec<i64> = Vec::new();

    for seed in seed_collection {
        let mut intermediate_value = seed;

        for map in &mappings {
            intermediate_value = map.map(intermediate_value);
        }

        target_locations.push(intermediate_value);
    }

    if let Some(min_value) = target_locations.iter().min() {
        println!("Lowest Number Location (Task One): {}", min_value); // Part One Answer
    } else {
        println!("The vector is empty.");
    }

    /************************************************************/
    //                          PART TWO                        //
    /************************************************************/

    let file: File = File::open(filename)?; // If open is error, halt program
    let reader = io::BufReader::new(file);
    let mut first_line = String::new();
    if let Some(result) = reader.lines().next() {
        first_line = result?;
    } else {
        println!("Read Failed");
    }

    let chunks: Vec<i64> = first_line
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut seed_collection: Vec<SeedBucket> = Vec::new(); // Part two

    for i in (0..chunks.len()).step_by(2) {
        let bucket = SeedBucket::new(chunks[i], chunks[i + 1]);
        seed_collection.push(bucket);
    }

    let mut min_location: Vec<i64> = Vec::new();

    for bucket in &mut seed_collection {
        let seed_end = &bucket.seed_start + &bucket.range;
        let mut minimum = std::i64::MAX;

        for seed in bucket.seed_start..seed_end {
            let mut intermediate_value = seed;

            for map in &mappings {
                intermediate_value = map.map(intermediate_value);
            }
            if intermediate_value < minimum {
                minimum = intermediate_value;
            }
        }

        min_location.push(minimum);
    }

    if let Some(min_value) = min_location.iter().min() {
        println!("Lowest Number Location (Task Two): {}", min_value); // Part One Answer
    } else {
        println!("The vector is empty.");
    }

    Ok(())
}

struct Mapping {
    rules: Vec<Rule>,
}

impl Mapping {
    fn new() -> Self {
        let rule_vec: Vec<Rule> = Vec::new();
        Self { rules: rule_vec }
    }

    fn load_rule(&mut self, dest_begin: i64, source_begin: i64, range: i64) {
        let rule = Rule::new(dest_begin, source_begin, range);
        self.rules.push(rule);
    }

    fn map(&self, input: i64) -> i64 {
        for rule in &self.rules {
            if input >= rule.left_boundary && input <= rule.right_boundary {
                return input + rule.offset;
            }
        }
        input
    }
}

struct Rule {
    left_boundary: i64,
    right_boundary: i64,
    offset: i64,
}

impl Rule {
    fn new(dest_begin: i64, source_begin: i64, range: i64) -> Self {
        let right_bound = source_begin + range;
        let difference = dest_begin - source_begin;

        Self {
            left_boundary: source_begin,
            right_boundary: right_bound,
            offset: difference,
        }
    }
}

struct SeedBucket {
    seed_start: i64,
    range: i64,
}

impl SeedBucket {
    fn new(start: i64, quantity: i64) -> Self {
        Self {
            seed_start: start,
            range: quantity,
        }
    }
}
