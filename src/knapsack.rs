#[derive(Debug)]
pub struct KnapsackProblem {
    pub n_items: u16,
    pub capacity: u16,
    pub treasure_items: Vec<KnapsackItem>,
}

impl KnapsackProblem {
    pub fn new(n_items: u16, capacity: u16, treasure_items: Vec<KnapsackItem>) -> KnapsackProblem {
        assert_eq!(
            treasure_items.len(),
            n_items as usize,
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
    pub value: u16,
    pub weight: u16,
}
