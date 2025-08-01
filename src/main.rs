mod file_reader;
mod knapsack;
mod strategy;

use knapsack::{KnapsackProblem};
use crate::knapsack::KnapsackSolution;
use crate::strategy::dynamic_programming_strategy;

fn main() {
    let filename = match file_reader::parse_args() {
        Some(filename) => filename,
        None => panic!("Please specify a valid filename using -file= CLI argument"),
    };
    let knapsack_problem: KnapsackProblem = file_reader::parse_input_file(filename);

    let soln: KnapsackSolution = dynamic_programming_strategy(knapsack_problem, true);
    println!("{}", soln);
}
