mod file_reader;
mod knapsack;
mod dynamic_programming;
mod knapsack_solver;
mod branch_and_bound;

use knapsack::{KnapsackProblem};
use crate::knapsack::KnapsackSolution;
use crate::dynamic_programming::{DynamicProgrammingSolver};
use crate::knapsack_solver::KnapsackSolver;

fn main() {
    let filename = match file_reader::parse_args() {
        Some(filename) => filename,
        None => panic!("Please specify a valid filename using -file= CLI argument"),
    };
    let knapsack_problem: KnapsackProblem = file_reader::parse_input_file(filename);

    let soln: KnapsackSolution = DynamicProgrammingSolver::solve(&knapsack_problem);
    println!("{}", soln);
}
