use std::fmt;

pub struct KnapsackSolution {
    pub obj: usize,
    pub opt: bool,
    pub selected_items: Vec<u8>,
}

impl fmt::Display for KnapsackSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}\n{}",
            self.obj,
            self.opt as u8,
            self.selected_items
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
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

    pub fn get_weights(&self) -> Vec<usize> {
        self.treasure_items.iter().map(|item| item.weight).collect()
    }
}
#[derive(Debug)]
pub struct KnapsackItem {
    pub value: usize,
    pub weight: usize,
}
