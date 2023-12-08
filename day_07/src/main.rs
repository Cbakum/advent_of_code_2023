use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // open file to search through data
    let filename: String = "data.txt".to_string();
    let file: File = File::open(filename)?; // If open is error, halt program
    let reader = io::BufReader::new(file);

    let mut game: Vec<Hand> = Vec::new();
    let mut card_mapping = create_card_map(); // Made mutable for part two

    // FOR PART TWO ONLY
    card_mapping.insert('J', 0);
    // COMMENT OUT FOR PART ONE

    // Iterate through each line
    for (_line_number, line) in reader.lines().enumerate() {
        let line = line?; // If line is error, halt program

        let cards: String = line.split(" ").nth(0).unwrap().trim().to_string();
        let wager: u64 = line.split(" ").last().unwrap().parse().unwrap();
        let hand = Hand::new(cards, wager, &card_mapping);
        game.push(hand);
    }

    game.sort_by(|a, b| a.strength.cmp(&b.strength));

    let mut total_winnings: u64 = 0;
    for (index, play) in game.iter().enumerate() {
        total_winnings += play.wager * ((index + 1) as u64);
    }

    println!("The total winnings are {}", total_winnings);

    Ok(())
}

fn create_card_map() -> HashMap<char, u32> {
    let mut map: HashMap<char, u32> = HashMap::new();
    map.insert('2', 1);
    map.insert('3', 2);
    map.insert('4', 3);
    map.insert('5', 4);
    map.insert('6', 5);
    map.insert('7', 6);
    map.insert('8', 7);
    map.insert('9', 8);
    map.insert('T', 9);
    map.insert('J', 10);
    map.insert('Q', 11);
    map.insert('K', 12);
    map.insert('A', 13);

    map
}

fn score_hand(hand: &str, card_mapping: &HashMap<char, u32>) -> (u32, u32) {
    let mut counts = HashMap::new();

    let mut weight: u32 = 0;
    let arbitrary_large_number = 50;
    for c in hand.chars() {
        *counts.entry(c).or_insert(0) += 1;
        weight = weight * arbitrary_large_number + card_mapping.get(&c).unwrap();
    }

    // FOR PART TWO
    let mut jokers = counts.remove(&'J').unwrap_or_default();

    let max_entry = match counts.iter().max_by_key(|&(_, val)| val) {
        Some(entry) => entry,
        None => {
            // Account for possibility of JJJJJ hand:
            (&'J', &0)
        }
    };

    jokers += max_entry.1;
    counts.insert(*max_entry.0, jokers);
    // COMMENT OUT FOR PART ONE

    let mut quantities: Vec<i32> = counts.values().cloned().collect();
    quantities.sort();

    let score: u32 = match quantities.as_slice() {
        &[1, 1, 1, 1, 1] => Hand::HIGH_CARD,
        &[1, 1, 1, 2] => Hand::ONE_PAIR,
        &[1, 2, 2] => Hand::TWO_PAIR,
        &[1, 1, 3] => Hand::THREE_OF_A_KIND,
        &[2, 3] => Hand::FULL_HOUSE,
        &[1, 4] => Hand::FOUR_OF_A_KIND,
        &[5] => Hand::FIVE_OF_A_KIND,
        _ => {
            println!("Hand Parsing failed");
            Hand::ERR_VALUE
        }
    };

    (score, weight)
}

struct Hand {
    wager: u64,
    strength: (u32, u32), // (Hand Strength, Weighted Card Strengths)
}

impl Hand {
    const HIGH_CARD: u32 = 0;
    const ONE_PAIR: u32 = 1;
    const TWO_PAIR: u32 = 2;
    const THREE_OF_A_KIND: u32 = 3;
    const FULL_HOUSE: u32 = 4;
    const FOUR_OF_A_KIND: u32 = 5;
    const FIVE_OF_A_KIND: u32 = 6;
    const ERR_VALUE: u32 = 99;

    fn new(hand: String, wager: u64, card_mapping: &HashMap<char, u32>) -> Self {
        let strength = score_hand(&hand, card_mapping);
        Self {
            wager: wager,
            strength: strength,
        }
    }
}
