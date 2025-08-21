use crate::knapsack::{KnapsackProblem, KnapsackSolution};
use crate::knapsack_solver::KnapsackSolver;

#[derive(Clone, Debug)]
pub struct BranchAndBoundNode {
    pub selected: Vec<u8>, //To determine the selected items at the optimal solution
    pub current_weight: usize, // Current weight at the current node (sum of all selected items)
    pub obj: usize,        // Value of selected items up to the current node
    pub best_relaxation: usize, // Calculate the relaxation with all remaining items set to true
}

// Add a converter to convert a BranchAndBoundNode to KnapsackSolution
pub struct BranchAndBoundSolver {}

impl KnapsackSolver for BranchAndBoundSolver {
    fn solve(&self, problem: &KnapsackProblem) -> KnapsackSolution {
        // TODO: Calc best relaxation depending on the strategy
        let best_relaxation: usize = Self::_calc_best_relaxation_unlimited_capacity(
            &problem,
            &(0..problem.n_items).collect::<Vec<usize>>(),
        );
        let mut best_node: BranchAndBoundNode = BranchAndBoundNode {
            selected: vec![],
            current_weight: 0,
            obj: 0,
            best_relaxation: best_relaxation,
        };

        //initialize the tree as a stack for depth-first search traversal
        let mut branch_and_bound_tree: Vec<BranchAndBoundNode> = vec![best_node.clone()];
        let mut nodes_explored = 0;
        while let Some(node) = branch_and_bound_tree.pop() {
            nodes_explored += 1;
            if cfg!(debug_assertions) {
                println!("node: {:?}, best_value: {}", node, best_node.obj);
            }

            let node_copy = node.clone();
            if node_copy.obj > best_node.obj {
                //This move does not result in any problems with the borrow checker, because
                //the node only has a vector (which has been cloned) and fields that implement Copy()
                best_node = node_copy;
            }

            if node.selected.len() == problem.n_items {
                continue;
            } //terminal node, no need to branch
            if node.best_relaxation < best_node.obj {
                continue;
            } //no need to explore this node, because the branch will never lead to a better solution

            //We do left traversal, so first put the right node (don't select item i + 1) on the stack
            let selected_items_right_node = {
                let mut v = node.selected.clone();
                v.push(0);
                v
            };
            let new_obj_right_node = node.obj;
            branch_and_bound_tree.push(BranchAndBoundNode {
                selected: selected_items_right_node,
                obj: new_obj_right_node,
                current_weight: node.current_weight,
                //TODO: Current best relaxation - value of the current item
                best_relaxation: new_obj_right_node
                    + Self::_calc_best_relaxation_unlimited_capacity(
                    &problem,
                    &(node.selected.len() + 1..problem.n_items).collect::<Vec<usize>>(),
                ),
            });
            let new_weight_left_node = node.current_weight
                + problem
                .treasure_items
                .get(node.selected.len())
                .unwrap()
                .weight;
            //Only add left node if the capacity is not yet exceeded
            if new_weight_left_node <= problem.capacity {
                let selected_items_left_node = {
                    let mut v = node.selected.clone();
                    v.push(1);
                    v
                };
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
                    //Unchanged, because we included the current item

                    // TODO: This may no longer hold in case of fractional relaxation
                    best_relaxation: node.best_relaxation,
                });
            }
        }

        if (cfg!(debug_assertions)) {
            println!("Nodes explored: {}", nodes_explored);
        }

        //assert whether item of best_node = equal to the number of items
        //Branch and bound will always find the optimal solution
        KnapsackSolution {
            obj: best_node.obj,
            opt: true,
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
        problem: &KnapsackProblem,
        current_best_relaxation: usize,
        current_weight: usize,
        remaining_items: &[usize],
    ) {
        //Fractional relaxation bounds are tighter, so we will explore fewer nodes
        //TODO: The sorting can be done before constructing the tree, which is more efficient.
        let mut sorted_items = problem.treasure_items[remaining_items].clone();
        sorted_items.sort_by(|x, y| {
            let ratio_x = x.value as f32 / x.weight as f32;
            let ratio_y = y.value as f32 / y.weight as f32;
            ratio_x.partial_cmp(&ratio_y).unwrap() //ascending order, because we pop
        });

        let mut best_relaxation = current_best_relaxation;
        let mut remaining_capacity = problem.capacity - current_weight;
        while let (Some(item)) = sorted_items.pop() {
            // TODO: Is this necessarily optimal? Because a lower ratio can still have a lower weight and fit the knapsack
            if item.weight <= remaining_capacity {
                best_relaxation += item.value;
                remaining_capacity -= item.weight;
            } else {
                break;
            }
        }

        //add fractional value of the highest ratio (if capacity left and items left)
        if remaining_capacity > 0 {
            if let (Some(item)) = sorted_items.pop() {
                best_relaxation += item.weight / remaining_capacity * item.value;
            }
        }

        best_relaxation
    }
}
