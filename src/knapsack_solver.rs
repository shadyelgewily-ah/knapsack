use crate::knapsack::{KnapsackProblem, KnapsackSolution};

pub trait KnapsackSolver {
    fn solve(&self, problem: &KnapsackProblem) -> KnapsackSolution;
}