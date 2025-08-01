
#[derive(Debug)]
pub struct KnapsackProblem {
    pub n_items: u16,
    pub capacity: u16,
    pub treasure_items: Vec<KnapsackItem>
}
#[derive(Debug)]
pub struct KnapsackItem {
    pub value: u16,
    pub weight: u16,
}
