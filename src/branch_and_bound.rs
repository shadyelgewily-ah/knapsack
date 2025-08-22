use crate::knapsack::{KnapsackItem, KnapsackProblem, KnapsackSolution};
use crate::knapsack_solver::KnapsackSolver;
use std::arch::aarch64::vsli_n_p8;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;

//TODO: Performance improvements:
// - only update the bound by using the previous bound + whether the item was selected or not
// (to avoid O(n) operations when scanning through the vector and making it O(1))
// No clones by using references, if possible.
//TODO: At any depth in the tree, there is some structure to the value of the bound, because
// we only recompute it partly (by using a different value and different remaining capacity)

//TODO: We can also cut off the process after 1 hour or so. Probably we have a very good solution by then.
//Another approach is to quit early when the best solution is within 1% of the best upper bound
#[derive(Clone, Debug)]
pub struct BranchAndBoundNode {
    pub selected: Vec<u8>, //To determine the selected items at the optimal solution
    pub current_weight: usize, // Current weight at the current node (sum of all selected items)
    pub obj: usize,        // Value of selected items up to the current node
    pub best_relaxation: usize, // Calculate the relaxation with all remaining items set to true
}

impl PartialEq for BranchAndBoundNode {
    fn eq(&self, other: &Self) -> bool {
        self.best_relaxation == other.best_relaxation
    }
}

impl Eq for BranchAndBoundNode {}

impl PartialOrd for BranchAndBoundNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.best_relaxation.partial_cmp(&other.best_relaxation)
    }
}

impl Ord for BranchAndBoundNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// Add a converter to convert a BranchAndBoundNode to KnapsackSolution
pub struct BranchAndBoundSolver {
    _nodes_explored: usize,
    _best_relaxation: usize,
    _optimality_perc: f32,
    _early_stopping_activated: bool,
}

impl Default for BranchAndBoundSolver {
    fn default() -> Self {
        Self {
            _nodes_explored: 0,
            _best_relaxation: 10000000,
            _optimality_perc: 0f32,
            _early_stopping_activated: false,
        }
    }
}

impl KnapsackSolver for BranchAndBoundSolver {
    fn solve(&mut self, problem: &KnapsackProblem) -> KnapsackSolution {
        if (cfg!(debug_assertions)) {
            println!("Solving with branch and bound...");
        }
        let start_time = Instant::now();

        let sorted_items = problem.get_best_value_per_weight_items();
        self._best_relaxation =
            Self::_calc_best_relaxation_fractionals(&sorted_items, &problem, 0, 0, 0);
        let mut best_node: BranchAndBoundNode = BranchAndBoundNode {
            selected: vec![],
            current_weight: 0,
            obj: 0,
            best_relaxation: self._best_relaxation,
        };

        //initialize the tree as a stack for depth-first search traversal
        // TODO: Make this configurable (but then how do we early stop?)
        //let mut branch_and_bound_tree: Vec<BranchAndBoundNode> = vec![];

        //Initialize the tree as a binary heap for best-first search traversal
        // The priority queue will grow enormous in memory if we don't periodically empty it
        // Therefore, we will periodically drain the binary heap and only retain promising nodes.
        let mut branch_and_bound_tree = BinaryHeap::new();
        branch_and_bound_tree.push(best_node.clone());

        self._nodes_explored = 0;
        while let Some(node) = branch_and_bound_tree.pop() {
            // TODO: Best relaxation only works with Priority Queue
            // For stack we need to keep track when that relaxation has already been explored
            // which is not trivial
            self._optimality_perc = (best_node.obj as f32 / node.best_relaxation as f32).min(1f32);

            if best_node.obj > 0 && self._nodes_explored % 100000 == 0 {
                branch_and_bound_tree = branch_and_bound_tree
                    .drain()
                    .filter(|node| node.best_relaxation >= best_node.obj)
                    .collect();
                if cfg!(debug_assertions) {
                    println!("Pruned the priority queue/stack to reduce memory consumption");
                    println!(
                        "Nodes explored: {}, Best value: {}, best upper bound: {}, optimality bound % {}",
                        self._nodes_explored,
                        best_node.obj,
                        node.best_relaxation,
                        self._optimality_perc
                    )
                }
            }

            if start_time.elapsed().as_secs() >= 120 {
                self._early_stopping_activated = true;
                if cfg!(debug_assertions) {
                    println!(
                        "Early stopping activated, good enough solution found or time exceeded."
                    );
                }
                break;
            }

            if node.obj > best_node.obj {
                //Only clone if needed
                best_node = node.clone();
            }

            if node.selected.len() == problem.n_items {
                continue;
            } //terminal node, no need to branch
            if node.best_relaxation <= best_node.obj {
                continue;
            } //no need to explore this node, because the branch will never lead to a better solution

            //TODO: This is not really 'nodes explored', improve this
            self._nodes_explored += 1;

            //We do right traversal, so we only need fewer copies
            let new_weight_left_node = node.current_weight
                + problem
                    .treasure_items
                    .get(node.selected.len())
                    .unwrap()
                    .weight;
            //Only add left node if the capacity is not yet exceeded
            if new_weight_left_node <= problem.capacity {
                let mut selected_items_left_node = node.selected.clone();
                selected_items_left_node.push(0);
                let new_obj_left_node = node.obj
                    + problem
                        .treasure_items
                        .get(node.selected.len())
                        .unwrap()
                        .value;
                branch_and_bound_tree.push(BranchAndBoundNode {
                    selected: selected_items_left_node,
                    obj: new_obj_left_node,
                    current_weight: new_weight_left_node,
                    best_relaxation: Self::_calc_best_relaxation_fractionals(
                        &sorted_items,
                        &problem,
                        new_obj_left_node,
                        new_weight_left_node,
                        node.selected.len() + 1,
                    ),
                });
            }
            let mut selected_items_right_node = node.selected; //Move, to avoid expensive copy of vector<u8>
            selected_items_right_node.push(0);
            let items_visited: usize = selected_items_right_node.len();
            branch_and_bound_tree.push(BranchAndBoundNode {
                selected: selected_items_right_node,
                obj: node.obj,
                current_weight: node.current_weight,
                best_relaxation: Self::_calc_best_relaxation_fractionals(
                    &sorted_items,
                    &problem,
                    node.obj,
                    node.current_weight,
                    items_visited,
                ),
            });
        }

        if (cfg!(debug_assertions)) {
            let elapsed = start_time.elapsed().as_secs();
            println!("Program ran in {:?} seconds", elapsed);
            println!(
                "Best score: {}, optimality perc: {}%, weight of knapsack: {}, capacity: {}",
                best_node.obj,
                self._optimality_perc * 100f32,
                best_node.current_weight,
                problem.capacity
            )
        }

        //Right pad with 0s in case the best solution is not a terminal node
        best_node.selected.resize(problem.n_items, 0);
        KnapsackSolution {
            obj: best_node.obj,
            opt: !self._early_stopping_activated,
            selected_items: best_node.selected,
        }
    }
}

impl BranchAndBoundSolver {
    fn _calc_best_relaxation_unlimited_capacity(
        problem: &KnapsackProblem,
        remaining_items: &[usize],
    ) -> usize {
        //These bounds are not very tight, therefore this can take a substantial amount of time.
        let mut best_relaxation: usize = 0;
        for item in remaining_items {
            best_relaxation += problem.treasure_items.get(*item).unwrap().value;
        }

        best_relaxation
    }

    fn _calc_best_relaxation_fractionals(
        sorted_items: &Vec<(usize, KnapsackItem)>,
        problem: &KnapsackProblem,
        current_value: usize,
        current_weight: usize,
        items_visited: usize,
    ) -> usize {
        // TODO: The subtle difference between sorting every decision node versus only once at the start
        // is that we must keep track which items are already selected, and only consider the remaining items
        // for updating the relaxation.
        // The example in the course where the first node gives partial relaxation 90 in my buggy solution
        // versus 92 in the example shows why things may go wrong. The real relaxation is 92,
        // but we think it is 90. So if we had found a solution with value 91, this branch would never be explored
        // even though the best solution can be 92.
        let mut best_relaxation = current_value;
        let mut remaining_capacity = problem.capacity - current_weight;

        for (original_idx, item) in sorted_items {
            if *original_idx < items_visited {
                continue; // this item can no longer be selected
            }
            if item.weight <= remaining_capacity {
                best_relaxation += item.value;
                remaining_capacity -= item.weight;
            } else if remaining_capacity > 0 {
                //add fractional value of the highest ratio (if capacity left and items left)
                best_relaxation +=
                    ((remaining_capacity as f32 / item.weight as f32) * item.value as f32) as usize;
                break;
            } else {
                break;
            }
        }

        best_relaxation
    }
}
