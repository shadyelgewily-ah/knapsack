use crate::knapsack::{KnapsackProblem, KnapsackItem};

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_args() -> Option<String> {
    env::args()
        .find(|arg| arg.starts_with("-file="))
        .map(|arg| arg["-file=".len()..].to_string())
}
pub fn parse_input_file(filename: String) -> KnapsackProblem {
    let file = File::open(filename).expect("Failed to open file");
    let mut lines = BufReader::new(file).lines();

    let header = lines
        .next()
        .expect("File is empty")
        .expect("Failed to read first line");
    let mut header_parts = header.split(" ");

    let n_items: u16 = header_parts
        .next()
        .expect("Missing number of items")
        .parse()
        .expect("Misformatted number of items");
    let capacity: u16 = header_parts
        .next()
        .expect("Missing knapsack capacity")
        .parse()
        .expect("Misformatted knapsack capacity");

    let mut treasure_items: Vec<KnapsackItem> = Vec::new();
    for line in lines {
        let line = line.expect("Invalid line");
        let mut line_parts = line.split(" ");

        let value: u16 = line_parts
            .next()
            .expect("Missing value")
            .parse()
            .expect("Invalid value");
        let weight: u16 = line_parts
            .next()
            .expect("Missing weight")
            .parse()
            .expect("Invalid weight");

        treasure_items.push(KnapsackItem { value, weight });
    }

    KnapsackProblem::new(n_items, capacity, treasure_items)
}