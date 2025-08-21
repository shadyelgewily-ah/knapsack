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

    pub fn get_best_value_per_weight_items(&self) -> Vec<KnapsackItem> {
        //Sort treasure items in descending order of value/weight
        let mut sorted_items: Vec<KnapsackItem> = self
            .treasure_items.clone()
            .iter()
            .map(|item| KnapsackItem {
                value: item.value,
                weight: item.weight,
            })
            .collect();

        sorted_items.sort_by(|x, y| {
            let ratio_x = x.value as f32 / x.weight as f32;
            let ratio_y = y.value as f32 / y.weight as f32;
            ratio_y.partial_cmp(&ratio_x).unwrap() //descending order
        });

        sorted_items
    }
}
#[derive(Debug, Clone)]
pub struct KnapsackItem {
    pub value: usize,
    pub weight: usize,
}
