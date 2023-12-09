use num_integer::lcm;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {

    // open file to search through data
    let filename: String = "data.txt".to_string();
    let file: File = File::open(filename)?; // If open is error, halt program
    let reader = io::BufReader::new(file);

    let mut directions: Vec<char> = Vec::new();
    let mut network: HashMap<String, (String, String)> = HashMap::new();

    // For part two
    let mut ghost_start_positions: Vec<String> = Vec::new();
    let mut lcm_accumulator: Vec<u64> = Vec::new();

    // Iterate through each line
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?; // If line is error, halt program

        if line_number > 1 {
            let location: String = line
                .split(" = ")
                .nth(0)
                .unwrap()
                .trim()
                .to_string();

            let left: String = line
                .split("(")
                .nth(1)
                .unwrap()
                .split(",")
                .nth(0)
                .unwrap()
                .trim()
                .to_string();

            let right: String = line
                .split(", ")
                .nth(1)
                .unwrap()
                .split(")")
                .nth(0)
                .unwrap()
                .trim()
                .to_string();

            if location.chars().nth(2).unwrap() == 'A' {
                // Collect all starting positions for ghosts
                ghost_start_positions.push(location.clone());
            }
        
            network.insert(location, (left, right));
        } 
        
        else {
            if !line.is_empty() {
                directions = line.trim().chars().collect();
            }
        }
    }

    for ghost in ghost_start_positions {
        // Navigate the network
        let mut navigator = Navigator::init(ghost, directions.len());

        while navigator.location.chars().nth(2).unwrap() != 'Z' {
            navigator.step(&directions, &network);
        }

        // Find all loop lengths for Zs
        println!("It took {} steps to get from AAA to {}", navigator.steps, navigator.location);
        lcm_accumulator.push(navigator.steps);
    }

    let arrival_time = lcm_of_vector(&lcm_accumulator);

    println!("It took {} steps for all ghosts to arrive at Zs", arrival_time);

    Ok(())
}

fn lcm_of_vector(numbers: &[u64]) -> u64 {
    numbers.iter().fold(1, |acc, &num| lcm(acc, num))
}
struct Navigator {
    steps: u64,
    location: String,
    curr_index: usize,
    max_index: usize,
}

impl Navigator {
    fn init(start_pos: String, vec_size: usize) -> Self {
        Self {
            steps: 0, // Start at zero
            location: start_pos,
            curr_index: 0, // Start at zero
            max_index: vec_size,
        }
    }

    fn step(&mut self, directions: &Vec<char>, map: &HashMap<String, (String, String)>) {
        let step_dir = match directions[self.curr_index] {
            'L' => 0 as usize,
            'R' => 1 as usize,
            _ => panic!("Unexpected direction"),
        };

        self.curr_index += 1;
        if self.curr_index == self.max_index {
            self.curr_index = 0;
        }

        let tuple = map.get(&self.location).unwrap();

        self.location = match step_dir {
            0 => tuple.0.clone(),
            1 => tuple.1.clone(),
            _ => panic!("Unexpected step direction"),
        };
        self.steps += 1;
    }
}