mod file_reader;
mod knapsack;

use knapsack::{KnapsackProblem};

fn main() {
    let filename = match file_reader::parse_args() {
        Some(filename) => filename,
        None => panic!("Please specify a valid filename using -file= CLI argument"),
    };
    let knapsack_problem: KnapsackProblem = file_reader::parse_input_file(filename);
    println!("{:?}", knapsack_problem)
}
