use crate::knapsack::{KnapsackItem, KnapsackProblem, KnapsackSolution};
use crate::knapsack_solver::KnapsackSolver;
use std::cmp::max;
use std::fmt;

#[derive(Debug)]
struct Matrix {
    data: Vec<usize>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize, val: usize) -> Self {
        Self {
            data: vec![val; rows * cols],
            rows,
            cols,
        }
    }

    fn set(&mut self, row: usize, col: usize, val: usize) {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        self.data[row * self.cols + col] = val;
    }

    fn get(&self, row: usize, col: usize) -> usize {
        self.data[row * self.cols + col]
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{:4} ", self.get(row, col))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct DynamicProgrammingSolver;

impl KnapsackSolver for DynamicProgrammingSolver {
    fn solve(problem: &KnapsackProblem) -> KnapsackSolution {
        if (problem.n_items * problem.capacity > 1_000_000) {
            panic!(
                "Only choose dynamic programming when n_items * capacity is reasonably small to avoid large memory footprint and slow runtime"
            );
        }

        let value_matrix = Self::fill_value_matrix(&problem);
        if cfg!(debug_assertions) {
            println!("{}", value_matrix);
        }
        // TODO: Determine which items were selected
        let selected_items = vec![];

        KnapsackSolution {
            obj: *value_matrix.data.last().unwrap(),
            //Dynamic programming always finds the optimal solution
            opt: true,
            selected_items,
        }
    }
}
impl DynamicProgrammingSolver {
    fn fill_value_matrix(problem: &KnapsackProblem) -> Matrix {
        let mut value_matrix: Matrix = Matrix::new(problem.capacity + 1, problem.n_items + 1, 0);
        for cur_item_no in 1..=problem.n_items {
            for cur_capacity in 1..=problem.capacity {
                let cur_item: &KnapsackItem = problem
                    .treasure_items
                    .get(cur_item_no - 1)
                    .expect("Could not load KnapsackItem from KnapsackProblem");
                if cur_item.weight <= cur_capacity {
                    let best_value_without_item = value_matrix.get(cur_capacity, cur_item_no - 1);
                    let best_value_with_item = cur_item.value
                        + value_matrix.get(cur_capacity - cur_item.weight, cur_item_no - 1);
                    value_matrix.set(
                        cur_capacity,
                        cur_item_no,
                        max(best_value_with_item, best_value_without_item),
                    );
                } else {
                    value_matrix.set(
                        cur_capacity,
                        cur_item_no,
                        value_matrix.get(cur_capacity, cur_item_no - 1),
                    )
                }
            }
        }
        value_matrix
    }
}
