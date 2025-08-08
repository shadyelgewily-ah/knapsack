use crate::knapsack::{KnapsackProblem, KnapsackSolution};
use crate::knapsack_solver::KnapsackSolver;

pub struct BranchAndBoundNode {
    pub selected: Vec<u8>, //To determine the selected items at the optimal solution
    pub item: usize, // we can use this to obtain the weights and values of the item (+ termination condition)
    pub obj: usize,  // Value of selected items up to the current node
    pub best_potential: f64, // Calculate the relaxation with all remaining items set to true
}

// Add a converter to convert a BranchAndBoundNode to KnapsackSolution
pub struct BranchAndBoundSolver {}

impl KnapsackSolver for BranchAndBoundSolver {
    fn solve(problem: &KnapsackProblem) -> KnapsackSolution {
        // TODO: Calc best relaxation depending on the strategy
        let best_relaxation: f64 = 0f64;
        let mut best_node: BranchAndBoundNode = BranchAndBoundNode {
            selected: vec![],
            item: 0,
            obj: 0,
            best_potential: best_relaxation,
        };

        //initialize stack for depth-first search (implemented as a Vector)
        let mut branch_and_bound_tree: Vec<BranchAndBoundNode> = vec![best_node];
        while let Some(node) = branch_and_bound_tree.pop() {
            if node.item == problem.n_items {
                continue;
            } //terminal node
            if node.best_potential < best_node.obj {
                continue;
            } //no need to explore this node, because the branch will never lead to a better solution

            //We do left traversal, so first put the right node (select item i + 1) on the stack
            let selected_items_right_node = {
                let mut v = node.selected.clone();
                v.push(0);
                v
            };
            branch_and_bound_tree.push(BranchAndBoundNode {
                selected: selected_items_right_node,
                item: node.item + 1,
                obj: node.obj + problem.treasure_items.get(node.item).unwrap().value,
                best_potential: 100f64, // Calculate new potential (problem.capacity - total weight as input)
            });
            let selected_items_left_node = {
                let mut v = node.selected.clone();
                v.push(1);
                v
            };
            branch_and_bound_tree.push(BranchAndBoundNode {
                selected: selected_items_left_node,
                item: node.item + 1,
                obj: node.obj,
                best_potential: 100f64,
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
