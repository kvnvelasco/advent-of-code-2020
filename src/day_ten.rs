use crate::utils::parse_input_into_vec;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{HashMap};
use std::fmt::{Debug};
use std::ops::{Add, AddAssign, Deref};
use std::rc::Rc;

// will transform a vector of [1, 2, 3, 4] to [1, 1, 1] where each item
// is the corresponding distance ot the previous item
fn compute_deltas_for_sorted_list(input_set: &Vec<isize>) -> Vec<isize> {
    let mut output = vec![];
    for (index, value) in input_set.iter().enumerate() {
        if let Some(next) = input_set.get(index + 1) {
            output.push(next - value)
        }
    }

    output
}

fn produce_sorted_joltage_list(source: &'static str) -> Vec<isize> {
    let mut inputs = parse_input_into_vec::<isize>(source);
    inputs.push(0);
    inputs.sort();
    let &max = inputs.last().unwrap();
    inputs.push(max + 3); // last item is always + 3 joltage
    inputs
}

/// Produces a hashmap where the keys are joltage differences and the values
/// are the counts of each difference
fn fold_up_differences_into_map(input: &Vec<isize>) -> HashMap<isize, isize> {
    input.iter().fold(HashMap::new(), |mut acc, &v| {
        acc.entry(v).or_insert(0).add_assign(1);
        acc
    })
}
#[derive(Debug)]
pub struct Node {
    value: isize,
    visited: bool,
    next: Vec<Rc<RefCell<Node>>>,
}

// there are occurances in the difference set that look like  [ 3,3 ] or [3, 3, 3]
// The comma in each set indicates a node that is  required to be in the final traversal
// this means that all possible traversals must pass through this node.
// We can chunk the graph by these nodes and multiply the traversal sizes together to get the output
fn chunk_nodes_by_pivot_nodes(source: &Vec<isize>) -> Vec<Vec<isize>> {
    let difference_set = compute_deltas_for_sorted_list(&source);
    let mut output = vec![];
    let mut holding = vec![0];
    for (index, delta) in difference_set.iter().enumerate() {
        let next = difference_set.get(index + 1).unwrap_or(delta);
        let &node = source.get(index + 1).unwrap();
        match (delta, next) {
            (3, 3) => {
                if holding.len() > 0 {
                    output.push(holding.clone())
                };
                output.push(vec![node]);
                holding.clear()
            }
            _ => holding.push(node),
        };
    }

    if holding.len() > 0 {
        output.push(holding.clone());
    }

    output
}

impl Node {
    pub fn construct_graph_from_vec(
        source: &Vec<isize>,
    ) -> (Vec<Rc<RefCell<Node>>>, Rc<RefCell<Node>>, Rc<RefCell<Node>>) {
        let nodes = source
            .iter()
            .map(|&v| {
                Rc::new(RefCell::new(Node {
                    next: vec![],
                    value: v,
                    visited: false,
                }))
            })
            .collect::<Vec<Rc<RefCell<Node>>>>();
        // starting node

        let socket = nodes.first().unwrap().clone();
        // terminal node
        let laptop = nodes.last().unwrap().clone();

        for (index, node) in nodes.iter().enumerate() {
            // we will find all of the possible nodes this one can point to
            let mut other_node_index = index + 1;
            loop {
                if let Some(other_node) = nodes.get(other_node_index) {
                    let distance =
                        other_node.as_ref().borrow().value - node.as_ref().borrow().value;

                    if distance <= 3 && distance > 0 {
                        other_node_index += 1;
                        // check for cycle
                        node.as_ref().borrow_mut().next.push(other_node.clone())
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        (nodes, socket, laptop)
    }
    // Returns a pointer to the socket, the laptop, and a list of all the nodes
    pub fn construct_graph_from_input(
        source: &'static str,
    ) -> (Vec<Rc<RefCell<Node>>>, Rc<RefCell<Node>>, Rc<RefCell<Node>>) {
        Self::construct_graph_from_vec(&produce_sorted_joltage_list(source))
    }
}

pub struct NodeTraverse {
    pub count: usize,
}

impl NodeTraverse {
    pub fn count_paths_from_to(
        &mut self,
        current_node: Rc<RefCell<Node>>,
        target: Rc<RefCell<Node>>,
    ) {
        if current_node.as_ptr() == target.as_ptr() {
            self.count += 1
        } else {
            for child in &current_node.as_ref().borrow().next {
                if child.as_ref().borrow().visited == true {
                    continue;
                }
                self.count_paths_from_to(child.clone(), target.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_ten::{
        chunk_nodes_by_pivot_nodes, compute_deltas_for_sorted_list, fold_up_differences_into_map,
        produce_sorted_joltage_list, Node, NodeTraverse,
    };
    
    

    #[test]
    fn test_data_a() {
        let inputs = produce_sorted_joltage_list(include_str!("inputs/day_ten.test.txt"));

        let difference_set = compute_deltas_for_sorted_list(&inputs);
        let map = fold_up_differences_into_map(&difference_set);

        assert_eq!(map.get(&1).unwrap() * map.get(&3).unwrap(), 220);
    }

    #[test]
    fn production_a() {
        let inputs = produce_sorted_joltage_list(include_str!("inputs/day_ten.txt"));

        let difference_set = compute_deltas_for_sorted_list(&inputs);
        let map = fold_up_differences_into_map(&difference_set);
        assert_eq!(map.get(&1).unwrap() * map.get(&3).unwrap(), 2343);
    }

    #[test]
    fn can_construct_graph() {
        let potato = produce_sorted_joltage_list(include_str!("inputs/day_ten.txt"));
        let chunks = chunk_nodes_by_pivot_nodes(&potato);
        let mut total = 1;
        for chunk in chunks.iter() {
            if chunk.len() == 1 {
                continue;
            }

            let (_all, start, end) = Node::construct_graph_from_vec(&chunk);

            let mut traverse = NodeTraverse { count: 0 };
            traverse.count_paths_from_to(start, end);
            total *= traverse.count;
        }

        assert_eq!(total, 31581162962944);
    }
}
