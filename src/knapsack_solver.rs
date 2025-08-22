use crate::knapsack::{KnapsackProblem, KnapsackSolution};

pub trait KnapsackSolver {
    fn solve(&mut self, problem: &KnapsackProblem) -> KnapsackSolution;
}