use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // initialize lookup table to search for numbers in string
    let mut table: LookupTable = load_table(); // Must be mutable for part two

    // ↓↓↓↓ For part two only ↓↓↓↓
    update_table(&mut table); // Comment this out for part one

    // open file to search through data
    let filename: String = "data.txt".to_string();
    let file: File = File::open(filename)?; // If open is error, halt program
    let reader = io::BufReader::new(file);

    // Keep running count for code
    let mut running_count: u32 = 0;

    // Iterate through each line
    for (_line_number, line) in reader.lines().enumerate() {
        let line = line?; // If line is error, halt program

        // Process each line here
        running_count += process_line(&line, &table);
    }

    println!("{running_count}");
    Ok(())
}

/// Searches a line for the first and last digit within it that is contained
/// in the lookup table
///
/// Takes these digits and returns a two digit number
fn process_line(line: &str, table: &LookupTable) -> u32 {
    let mut first: Entry = Entry::new();
    let mut last: Entry = Entry::new();

    // Change from default to search for highest index for last
    last.index = 0;

    // Check all elements in lookup table for each line
    for &(ref text, number) in &table.elements {
        if let Some(index) = find_substring_first(line, text) {
            if index < first.index {
                first.number = number;
                first.index = index;
            }
        }
        if let Some(index) = find_substring_last(line, text) {
            if index >= last.index {
                last.number = number;
                last.index = index;
            }
        }
    }

    // Return two digit number
    (first.number * 10) + last.number
}

/// Finds the index of the first instance of a string slice within a line.
///
/// Uses an option to return Some(index) if found and None otherwise
fn find_substring_first(line: &str, element: &str) -> Option<u32> {
    match line.find(element) {
        Some(index) => Some(index as u32),
        None => None,
    }
}

/// Finds the index of the last instance of a string slice within a line.
///
/// Uses an option to return Some(index) if found and None otherwise
fn find_substring_last(line: &str, element: &str) -> Option<u32> {
    match line.rfind(element) {
        Some(index) => Some(index as u32),
        None => None,
    }
}

/// Creates a lookup table to lookup string slices within each line
///
/// Maps each string of a number to its digit
fn load_table() -> LookupTable {
    let table: LookupTable = LookupTable {
        elements: vec![
            ("1".to_string(), 1),
            ("2".to_string(), 2),
            ("3".to_string(), 3),
            ("4".to_string(), 4),
            ("5".to_string(), 5),
            ("6".to_string(), 6),
            ("7".to_string(), 7),
            ("8".to_string(), 8),
            ("9".to_string(), 9),
        ],
    };
    table
}

/// Takes the table from part one and updates it with spelled versions of numbers
///
/// Requires the original table as a reference
fn update_table(table: &mut LookupTable) {
    let new_elements = vec![
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ];

    table.elements.extend(new_elements);
}

struct Entry {
    number: u32,
    index: u32,
}

impl Entry {
    /// Populate entry with default values
    fn new() -> Self {
        Entry {
            number: 0,
            index: std::u32::MAX,
        }
    }
}

struct LookupTable {
    elements: Vec<(String, u32)>,
}
