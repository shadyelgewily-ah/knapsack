use std::fmt;

pub struct KnapsackSolution {
    pub obj: usize,
    pub opt: bool,
}

impl fmt::Display for KnapsackSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.obj, self.opt as u8);
        Ok(())
    }
}

#[derive(Debug)]
pub struct KnapsackProblem {
    pub n_items: usize,
    pub capacity: usize,
    pub treasure_items: Vec<KnapsackItem>,
}

impl KnapsackProblem {
    pub fn new(
        n_items: usize,
        capacity: usize,
        treasure_items: Vec<KnapsackItem>,
    ) -> KnapsackProblem {
        assert_eq!(
            treasure_items.len(),
            n_items,
            "Size of treasure items does not match n_items"
        );
        KnapsackProblem {
            n_items,
            capacity,
            treasure_items,
        }
    }
}
#[derive(Debug)]
pub struct KnapsackItem {
    pub value: usize,
    pub weight: usize,
}
