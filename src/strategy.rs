use std::fmt;
use crate::knapsack::KnapsackProblem;

#[derive(Debug)]
struct Matrix {
    data: Vec<i32>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize, val: i32) -> Self {
        Self {
            data: vec![val; rows * cols],
            rows,
            cols,
        }
    }

    fn set(&mut self, row: usize, col: usize, val: i32) {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        self.data[row * self.cols + col] = val;
    }

    fn get(&self, row: usize, col: usize) -> i32 {
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

pub fn dynamic_programming_strategy(problem: KnapsackProblem) {
    if (problem.n_items > 100) | (problem.capacity > 100) {
        panic!("Only choose dynamic programming when n_items * capacity is reasonably small");
    }

    let mut value_matrix: Matrix = Matrix::new(
        (problem.capacity + 1) as usize,
        (problem.n_items + 1) as usize,
        0,
    );
    println!("{}", value_matrix);
}
