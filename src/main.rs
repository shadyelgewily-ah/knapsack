mod file_reader;
mod knapsack;
mod dynamic_programming;
mod branch_and_bound;
mod knapsack_solver;


use knapsack::{KnapsackProblem};
use crate::knapsack::KnapsackSolution;
use crate::branch_and_bound::{BranchAndBoundSolver};
use crate::dynamic_programming::{DynamicProgrammingSolver};
use crate::knapsack_solver::KnapsackSolver;

fn main() {
    let filename = match file_reader::parse_args() {
        Some(filename) => filename,
        None => panic!("Please specify a valid filename using -file= CLI argument"),
    };
    let knapsack_problem: KnapsackProblem = file_reader::parse_input_file(filename);

    let problem_size = knapsack_problem.n_items * knapsack_problem.capacity;

    let solver: Box<dyn KnapsackSolver> = match problem_size {
        size if size < 1_000_000 => Box::new(BranchAndBoundSolver {}),
        _ => Box::new(DynamicProgrammingSolver {}),
    };
    let soln: KnapsackSolution = solver.solve(&knapsack_problem);
    println!("{}", soln);
}
