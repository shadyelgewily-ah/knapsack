mod file_reader;
mod knapsack;
mod dynamic_programming;

use knapsack::{KnapsackProblem};
use crate::knapsack::KnapsackSolution;
use crate::dynamic_programming::{DynamicProgrammingStrategy};

fn main() {
    let filename = match file_reader::parse_args() {
        Some(filename) => filename,
        None => panic!("Please specify a valid filename using -file= CLI argument"),
    };
    let knapsack_problem: KnapsackProblem = file_reader::parse_input_file(filename);

    let soln: KnapsackSolution = DynamicProgrammingStrategy::solve(&knapsack_problem);
    println!("{}", soln);
}
