use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {

    /************************************************************/
    //                          PART ONE                        //
    /************************************************************/

    // open file to search through data
    let filename: String = "data.txt".to_string();
    let file: File = File::open(filename)?; // If open is error, halt program
    let reader = io::BufReader::new(file);

    let mut directions: Vec<char> = Vec::new();
    let mut network: HashMap<String, (String, String)> = HashMap::new();

    // Iterate through each line
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?; // If line is error, halt program

        if line_number > 1 {
            let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

            if let Some(captures) = re.captures(&line) {
                // Extract the three strings from the captures
                let location = captures[1].to_string();
                let left_pos = captures[2].to_string();
                let right_pos = captures[3].to_string();

                network.insert(location, (left_pos, right_pos));
            }
            else {
                println!("Regex failed");
                break;
            }
        }
        else {
            if !line.is_empty() {
                directions = line.trim().chars().collect();
            }
        }
    }

    // Navigate the network
    let mut navigator = Navigator::init(directions, network);
    let destination = "ZZZ".to_string();

    while navigator.location != destination {
        navigator.step();
    }

    println!("It took {} steps to get from AAA to ZZZ", navigator.steps);

    

    Ok(())
}

struct Navigator {
    steps: u32,
    location: String,
    curr_index: usize,
    max_index: usize,
    directions: Vec<char>,
    network: HashMap<String, (String, String)>
}

impl Navigator {
    fn init(directions: Vec<char>, map: HashMap<String, (String, String)>) -> Self {
        let vec_size = directions.len();
        let loc = "AAA".to_string(); // Start location

        Self {
            steps: 0, // Start at zero
            location: loc,
            curr_index: 0, // Start at zero
            max_index: vec_size,
            directions: directions,
            network: map
        }
    }

    fn step(&mut self) {
        let step_dir = match self.directions[self.curr_index] {
            'L' => 0 as usize,
            'R' => 1 as usize,
            _ => panic!("Unexpected direction"),
        };
        
        self.curr_index += 1;
        if self.curr_index == self.max_index {
            self.curr_index = 0;
        }
        
        let tuple = self.network.get(&self.location).unwrap();

        self.location = match step_dir {
            0 => tuple.0.clone(),
            1 => tuple.1.clone(),
            _ => panic!("Unexpected step direction"),
        };
        self.steps += 1;
        


    }

}