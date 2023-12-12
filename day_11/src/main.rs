use std::collections::HashSet;
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

    let mut space = Space::new();
    let expansion_factor_pt1: i64 = 2;

    // For part two
    let mut space2 = Space::new();
    let expansion_factor_pt2: i64 = 1000000;

    // Iterate through each line
    for (_line_number, line) in reader.lines().enumerate() {
        let line = line?; // If line is error, halt program

        let row: Vec<char> = line.trim().chars().collect();
        if !row.is_empty() {
            space.grid.push(row.clone());
            // For part two
            space2.grid.push(row);
        }
    }

    space.expand_space(expansion_factor_pt1);
    space.find_galaxies();

    let total_distance = space.get_all_distances();

    println!("The total distance is {}", total_distance);

    /************************************************************/
    //                          PART TWO                        //
    /************************************************************/
    
    space2.find_galaxies();
    space2.expand_space(expansion_factor_pt2);

    let total_distance = space2.get_all_distances();

    println!("The total expanded distance is {}", total_distance);

    Ok(())
}

struct Space {
    grid: Vec<Vec<char>>,
    galaxies: Vec<(usize, usize)>,
    expansion_factor: i64,
    expansive_rows: HashSet<usize>,
    expansive_columns: HashSet<usize>,
}

impl Space {
    fn new() -> Self {
        Self {
            grid: Vec::new(),
            galaxies: Vec::new(),
            expansion_factor: 1,
            expansive_rows: HashSet::new(),
            expansive_columns: HashSet::new(),
        }
    }

    fn find_galaxies(&mut self) {
        let galaxy = '#';
        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, item) in row.iter().enumerate() {
                if item == &galaxy {
                    self.galaxies.push((col_index, row_index));
                }
            }
        }
    }

    fn expand_space(&mut self, expanded_size: i64) {
        let galaxy: char = '#';
        self.expansion_factor = expanded_size - 1;

        // Expand space in Y direction
        for (row_coord, row) in self.grid.iter().enumerate() {
            if !row.contains(&galaxy) {
                self.expansive_rows.insert(row_coord);
            }
        }

        // Expand space in the X direction
        self.expansive_columns = (0..self.grid[0].len())
            .filter(|&col_index| !self.grid.iter().any(|row| row[col_index] == galaxy))
            .collect();
    }

    fn get_distance(&self, galaxy1: (usize, usize), galaxy2: (usize, usize)) -> i64 {
        let y1 = galaxy1.0 as i64;
        let y2 = galaxy2.0 as i64;
        let x1 = galaxy1.1 as i64;
        let x2 = galaxy2.1 as i64;

        let x_expanse: i64 = ((galaxy1.0).min(galaxy2.0)..(galaxy1.0).max(galaxy2.0))
            .filter(|&num| self.expansive_columns.contains(&num))
            .count() as i64;

        let y_expanse: i64 = ((galaxy1.1).min(galaxy2.1)..(galaxy1.1).max(galaxy2.1))
            .filter(|&num| self.expansive_rows.contains(&num))
            .count() as i64;

        let distance = (y2 - y1).abs()
            + (x2 - x1).abs()
            + x_expanse * self.expansion_factor
            + y_expanse * self.expansion_factor;

        distance
    }

    fn get_all_distances(&self) -> i64 {
        let mut total_sum: i64 = 0;

        for index1 in 0..(self.galaxies.len() - 1) {
            for index2 in (index1 + 1)..self.galaxies.len() {
                total_sum += Space::get_distance(self, self.galaxies[index1], self.galaxies[index2]);
            }
        }

        total_sum
    }
}
