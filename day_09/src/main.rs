use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // open file to search through data
    let filename: String = "data.txt".to_string();
    let file: File = File::open(filename)?; // If open is error, halt program
    let reader = io::BufReader::new(file);

    let mut accumulate_right: i32 = 0;
    let mut accumulate_left: i32 = 0;

    // Iterate through each line
    for (_line_number, line) in reader.lines().enumerate() {
        let line = line?; // If line is error, halt program

        let sensor_history: Vec<i32> = line
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let mut history = Sequence::new(sensor_history.clone());
        history.find_difference_recursive(&sensor_history);                                           
        let last_row_index = &history.difference.len() - 1;

        history.bubble_up_right(last_row_index);
        history.bubble_up_left(last_row_index);

        accumulate_right += history.difference[0].last().unwrap();
        accumulate_left += history.difference[0].first().unwrap();
    }
    
    println!("The total sum of future values is {}", accumulate_right);
    println!("The total sum of prior values is {}", accumulate_left);



    Ok(())
}

fn find_difference(list: &Vec<i32>) -> Vec<i32> {
    let mut difference: Vec<i32> = Vec::new();

    for i in 0..(list.len() - 1) {
        let delta: i32 = list[i + 1] - list[i];
        difference.push(delta);
    }
    difference
}

struct Sequence {
    difference: Vec<Vec<i32>>,
}

impl Sequence {
    fn new(history: Vec<i32>) -> Self {
        Self { difference: vec![history] }
    }  

    fn find_difference_recursive(&mut self, list: &Vec<i32>) {
        let delta = find_difference(list);

        if !delta.iter().all(|&x| x == 0) {
            self.difference.push(delta.clone());
            self.find_difference_recursive(&delta);
        }
    }

    fn bubble_up_right(&mut self, lower: usize) {
        let upper: usize = lower - 1;

        let next_item: i32 = self.difference[lower].last().unwrap() + self.difference[upper].last().unwrap();

        self.difference[upper].push(next_item);

        if upper != 0 {
            self.bubble_up_right(upper);
        }
    }

    fn bubble_up_left(&mut self, lower: usize) {
        let upper: usize = lower - 1;

        let prior_item: i32 = self.difference[upper].first().unwrap() - self.difference[lower].first().unwrap();

        self.difference[upper].insert(0, prior_item);

        if upper != 0 {
            self.bubble_up_left(upper);
        }
    }
}
