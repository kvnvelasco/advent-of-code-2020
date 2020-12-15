use crate::utils::{split_into_array_by, split_once_at};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialOrd, PartialEq)]
struct InputLine {
    target_bag: &'static str,
    containing_children: Vec<InputChild>,
}

impl InputLine {
    // basic queries about input line internals
    fn contains_bag_type(&self, bag_type: &'static str) -> bool {
        self.containing_children
            .iter()
            .find(|c| c.name == bag_type)
            .is_some()
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct InputChild {
    count: usize,
    name: &'static str,
}

// light red bags contain 1 bright white bag, 2 muted yellow bag.
fn parse_input_line(input_line: &'static str) -> InputLine {
    let (name, rest) = split_once_at(input_line, "bags contain");
    let (rest, _) = split_once_at(rest, ".");
    let rules = if rest.contains("no other bags") {
        vec![]
    } else {
        split_into_array_by(rest, ", ")
            .iter()
            .map(|&rule| {
                let (count, name) = split_once_at(rule.trim(), " ");
                let (name, _) = split_once_at(name, " bag");
                InputChild {
                    count: count.parse().expect("Unable to parse count"),
                    name,
                }
            })
            .collect()
    };

    InputLine {
        containing_children: rules,
        target_bag: name.trim(),
    }
}

#[derive(Debug)]
struct RuleSearch {
    rule_map: HashMap<&'static str, InputLine>,
}

impl RuleSearch {
    fn from_input_lines(input_lines: impl Iterator<Item = InputLine>) -> Self {
        let mut map = HashMap::new();

        for line in input_lines {
            map.insert(line.target_bag, line);
        }

        Self { rule_map: map }
    }
    fn search_for_all_bags_that_contain_bag(
        &self,
        bag_type: &'static str,
    ) -> HashSet<&'static str> {
        let mut set = HashSet::new();
        for (bag_name, line) in self.rule_map.iter() {
            if line.contains_bag_type(&bag_type) {
                set.insert(&**bag_name);
            }
        }

        set
    }

    fn search_for_all_bags_that_can_eventually_contain(
        &self,
        bag_type: &'static str,
    ) -> HashSet<&'static str> {
        let mut output_set = self.search_for_all_bags_that_contain_bag(&bag_type);
        // println!("{:?} direclty contains {}", &output_set, &bag_type);
        let mut holding_set = output_set.clone();
        loop {
            let mut interim_set = HashSet::new();
            for item in holding_set.iter() {
                let containing_bags = self.search_for_all_bags_that_contain_bag(&*item);

                // println!("{:?} contain {}", &containing_bags, item);
                output_set = output_set.union(&containing_bags).map(|x| *x).collect();
                interim_set = interim_set.union(&containing_bags).map(|x| *x).collect()
            }
            if interim_set.is_empty() {
                break;
            } else {
                holding_set = interim_set
            }
        }

        output_set
    }

    fn count_total_number_of_bags_from(&self, target_bag: &'static str) -> usize {
        let target = self.rule_map.get(target_bag).unwrap();

        let mut sum = 1;

        for child in target.containing_children.iter() {
            sum += child.count * self.count_total_number_of_bags_from(child.name)
        }

        sum
    }

    fn count_number_of_bags_nested_in(&self, target_bag: &'static str) -> usize {
        self.count_total_number_of_bags_from(target_bag) - 1 // subtract the starting bag
    }
}

#[cfg(test)]
mod tests {
    use crate::day_seven::{parse_input_line, InputChild, InputLine, RuleSearch};
    use std::collections::{HashMap, HashSet};
    use std::rc::Rc;

    #[test]
    fn it_can_parse_an_input_rule() {
        let expected = InputLine {
            target_bag: "light red",
            containing_children: vec![
                InputChild {
                    count: 1,
                    name: "bright white",
                },
                InputChild {
                    count: 2,
                    name: "muted yellow",
                },
            ],
        };
        assert_eq!(
            parse_input_line("light red bags contain 1 bright white bag, 2 muted yellow bag."),
            expected
        );
    }

    #[test]
    fn search_struct_can_be_initialised() {
        let search_struct = RuleSearch::from_input_lines(
            include_str!("inputs/day_seven.test.txt")
                .lines()
                .map(parse_input_line),
        );
    }

    #[test]
    fn search_struct_can_find_all_bags_that_contain_a_bag() {
        let search_struct = RuleSearch::from_input_lines(
            include_str!("inputs/day_seven.test.txt")
                .lines()
                .map(parse_input_line),
        );
        let finished_set = search_struct.search_for_all_bags_that_contain_bag("dotted black");

        assert_eq!(finished_set.len(), 2);
        assert!(finished_set.contains("vibrant plum"));
        assert!(finished_set.contains("dark olive"));
    }

    #[test]
    fn search_struct_can_find_top_level_bags() {
        let search_struct = RuleSearch::from_input_lines(
            include_str!("inputs/day_seven.test.txt")
                .lines()
                .map(parse_input_line),
        );
        let finished_set =
            search_struct.search_for_all_bags_that_can_eventually_contain("shiny gold");

        assert_eq!(finished_set.len(), 4);
    }

    #[test]
    fn production_run_a() {
        let search_struct = RuleSearch::from_input_lines(
            include_str!("inputs/day_seven.txt")
                .lines()
                .map(parse_input_line),
        );
        let finished_set =
            search_struct.search_for_all_bags_that_can_eventually_contain("shiny gold");

        assert_eq!(finished_set.len(), 211);
    }

    #[test]
    fn can_count_the_number_of_bags_in() {
        let search_struct = RuleSearch::from_input_lines(
            include_str!("inputs/day_seven_b.test.txt")
                .lines()
                .map(parse_input_line),
        );
        let finished_set = search_struct.count_number_of_bags_nested_in("shiny gold");

        assert_eq!(finished_set, 126);
    }

    #[test]
    fn can_count_the_number_of_bags_in_2() {
        let search_struct = RuleSearch::from_input_lines(
            include_str!("inputs/day_seven.test.txt")
                .lines()
                .map(parse_input_line),
        );
        let finished_set = search_struct.count_number_of_bags_nested_in("shiny gold");

        assert_eq!(finished_set, 32);
    }

    #[test]
    fn production_run_b() {
        let search_struct = RuleSearch::from_input_lines(
            include_str!("inputs/day_seven.txt")
                .lines()
                .map(parse_input_line),
        );
        let finished_set = search_struct.count_number_of_bags_nested_in("shiny gold");

        assert_eq!(finished_set, 211);
    }
}
