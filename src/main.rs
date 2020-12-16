#[macro_use]
extern crate indoc;

use crate::day_ten::{Node, NodeTraverse};

mod utils;

mod day_eight;
mod day_five;
mod day_four;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_ten;
mod day_three;
mod day_two;

fn main() {
    let (_all, start, end) = Node::construct_graph_from_input(include_str!("inputs/day_ten.txt"));

    let mut traverse = NodeTraverse { count: 0 };
    traverse.count_paths_from_to(start, end);
}
