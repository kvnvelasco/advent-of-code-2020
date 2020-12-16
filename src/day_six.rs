use std::collections::{HashMap, HashSet};
use std::str::Lines;

#[derive(Debug)]
struct GroupIterator {
    source: Lines<'static>,
}

impl GroupIterator {
    fn new(source: &'static str) -> Self {
        Self {
            source: source.lines(),
        }
    }
}

impl Iterator for GroupIterator {
    type Item = Group;

    fn next(&mut self) -> Option<Self::Item> {
        let mut group_members = vec![];
        loop {
            let next = self.source.next();
            if next.is_none() && group_members.len() > 0 {
                break;
            } else if next.is_none() {
                return None;
            }

            let next = next.unwrap();

            if next.is_empty() {
                break;
            }
            group_members.push(next);
        }

        Some(Group {
            lines: group_members,
            index: 0,
        })
    }
}

#[derive(Debug)]
struct Group {
    lines: Vec<&'static str>,
    index: usize,
}

impl Iterator for Group {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.lines.get(self.index);
        self.index += 1;

        output.map(|x| *x)
    }
}

fn determine_answer_set_for_group(group: &mut Group) -> HashSet<char> {
    let mut answer_set = HashSet::new();
    for person in group {
        for answer in person.chars() {
            answer_set.insert(answer);
        }
    }
    answer_set
}

fn determine_common_answer_set_for_group(group: &mut Group) -> HashSet<char> {
    let mut group_map = HashMap::new();
    let group_size = group.lines.len();
    for person in group {
        for answer in person.chars() {
            let value = group_map.entry(answer).or_insert(0usize);
            *value += 1;
        }
    }

    let mut output_set = HashSet::new();
    for (answer, _count) in group_map.iter().filter(|(_c, x)| **x == group_size) {
        output_set.insert(*answer);
    }

    output_set
}

#[cfg(test)]
mod tests {
    use crate::day_six::{
        determine_answer_set_for_group, determine_common_answer_set_for_group, Group, GroupIterator,
    };
    use indoc::indoc;

    #[test]
    fn it_can_parse_inputs() {
        let input = indoc! {r#"
            a
            b
            c
            
            abc
            
            abcx"#};

        let group_iterator = GroupIterator::new(input);

        let vec = group_iterator.collect::<Vec<Group>>();
        assert_eq!(vec.len(), 3);

        assert_eq!(vec[0].lines, ["a", "b", "c"]);
        assert_eq!(vec[1].lines, ["abc"]);
        assert_eq!(vec[2].lines, ["abcx"]);
    }

    #[test]
    fn it_can_determine_unique_answer_sets() {
        let mut group = Group {
            index: 0,
            lines: vec!["abc", "ab", "x", "y"],
        };

        let set = determine_answer_set_for_group(&mut group);

        for value in vec!['a', 'b', 'c', 'x', 'y'] {
            assert!(set.get(&value).is_some());
        }

        assert_eq!(set.len(), 5);
    }

    #[test]
    fn test_data_for_a() {
        let input = indoc! {r#"
            abc

            a
            b
            c
            
            ab
            ac
            
            a
            a
            a
            a
            
            b"#};

        let length: usize = GroupIterator::new(input)
            .map(|mut x| determine_answer_set_for_group(&mut x))
            .map(|x| x.len())
            .sum();

        assert_eq!(length, 11);
    }

    #[test]
    fn production_a() {
        let input = include_str!("inputs/day_six.txt");

        let length: usize = GroupIterator::new(input)
            .map(|mut x| determine_answer_set_for_group(&mut x))
            .map(|x| x.len())
            .sum();

        assert_eq!(length, 6542);
    }

    #[test]
    fn test_data_for_b() {
        let input = indoc! {r#"
            abc

            a
            b
            c
            
            ab
            ac
            
            a
            a
            a
            a
            
            b"#};

        let length: usize = GroupIterator::new(input)
            .map(|mut x| determine_common_answer_set_for_group(&mut x))
            .map(|x| x.len())
            .sum();

        assert_eq!(length, 6);
    }

    #[test]
    fn production_b() {
        let input = include_str!("inputs/day_six.txt");

        let length: usize = GroupIterator::new(input)
            .map(|mut x| determine_common_answer_set_for_group(&mut x))
            .map(|x| x.len())
            .sum();

        assert_eq!(length, 3299);
    }
}
