use crate::knapsack::{KnapsackProblem, KnapsackSolution};
use crate::knapsack_solver::KnapsackSolver;

#[derive(Clone)]
pub struct BranchAndBoundNode {
    pub selected: Vec<u8>, //To determine the selected items at the optimal solution
    pub obj: usize,        // Value of selected items up to the current node
    pub best_relaxation: usize, // Calculate the relaxation with all remaining items set to true
}

// Add a converter to convert a BranchAndBoundNode to KnapsackSolution
pub struct BranchAndBoundSolver {}

impl KnapsackSolver for BranchAndBoundSolver {
    fn solve(&self, problem: &KnapsackProblem) -> KnapsackSolution {
        // TODO: Calc best relaxation depending on the strategy
        let best_relaxation: usize = Self::_calc_best_relaxation_unlimited_capacity(&problem, &(1..=problem.n_items).collect::<Vec<usize>>());
        let mut best_node: BranchAndBoundNode = BranchAndBoundNode {
            selected: vec![],
            obj: 0,
            best_relaxation: best_relaxation,
        };

        //initialize the tree as a stack for depth-first search traversal
        let mut branch_and_bound_tree: Vec<BranchAndBoundNode> = vec![best_node.clone()];
        while let Some(node) = branch_and_bound_tree.pop() {
            if node.selected.len() == problem.n_items {
                continue;
            } //terminal node
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
                //try to simplify this
                best_relaxation: new_obj_right_node
                    + Self::_calc_best_relaxation_unlimited_capacity(
                    &problem,
                    &(node.selected.len()+1..=problem.n_items)
                        .collect::<Vec<usize>>(),
                ),
            });
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
                obj: node.obj,
                best_relaxation: new_obj_left_node
                    + Self::_calc_best_relaxation_unlimited_capacity(
                    &problem,
                    &(node.selected.len()+1..=problem.n_items).collect::<Vec<usize>>(),
                ),
            });

            if node.obj > best_node.obj {
                //This move does not result in any problems with the borrow checker, because
                //the node only has a vector (which has been cloned) and fields that implement Copy()
                best_node = node;
            }
        }

        //assert whether item of best_node = equal to the number of items
        //Branch and bound will always find the optimal solution
        KnapsackSolution {
            obj: best_node.obj,
            opt: true,
            selected_items: vec![],
        }
    }
}

impl BranchAndBoundSolver {
    fn _calc_best_relaxation_unlimited_capacity(
        problem: &KnapsackProblem,
        remaining_items: &[usize],
    ) -> usize {
        let mut best_relaxation: usize = 0;
        for item in remaining_items {
            best_relaxation += problem.treasure_items.get(*item - 1).unwrap().value;
        }

        best_relaxation
    }

    fn _calc_best_relaxation_fractionals(
        problem: &KnapsackProblem,
        selected: Vec<u8>,
        remaining_items: &[usize],
    ) {
        //todo: sort slice of vector with the remaining items in the order of value/weight
        //Perhaps the algorithm is more efficient (but can become more complicated),
        // if we do this sorting before constructing the tree
        // Calculate remaining capacity by summing the weights of selected and subtracting from capacity
        // initialize potential = 0
        // while potential + weight of current item <= remaining_capacity:
        // potential += value of current item
        // final step: potential += ( weight_of_current_item // remaining capacity) * value of curren item
        // return potential
    }
}
