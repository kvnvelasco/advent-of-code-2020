use std::collections::HashSet;
use std::error::Error;

fn a() -> Result<usize, Box<dyn Error>> {
    let mut input = include_str!("./inputs/day1.txt")
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let set_minimum = input.iter().min().unwrap().clone();
    // we can exclude all items that are greater than 2020 - minimum because there's no way to satisfy
    // that sum

    input = input
        .into_iter()
        .filter(|&x| 2020 - x >= set_minimum)
        .collect();

    for left_position in 0..input.len() {
        for right_position in left_position + 1..input.len() {
            let (left, right) = (input[left_position], input[right_position]);
            if left + right == 2020 {
                return Ok(left * right);
            }
        }
    }
    return Err("Unable to find matching pattern".into());
}

fn b() -> Result<usize, Box<dyn Error>> {
    // Produces an iterator that returns one line at a time with each input parsed as a
    // usize (native integer size)
    let mut input = include_str!("./inputs/day1.txt")
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let set_minimum = input.iter().min().unwrap().clone();
    // we can exclude all items that are greater than 2020 - minimum because there's no way to satisfy
    // that sum
    input = input
        .into_iter()
        .filter(|&x| 2020 - x >= set_minimum)
        .collect();

    for left_position in 0..input.len() {
        for middle_position in left_position + 1..input.len() {
            for right_position in middle_position + 1..input.len() {
                let (left, middle, right) = (
                    input[left_position],
                    input[middle_position],
                    input[right_position],
                );
                if left + middle + right == 2020 {
                    return Ok(left * middle * right);
                }
            }
        }
    }

    return Err("Unable to find matching pattern".into());
}

pub fn find_pair_of_numbers_with_sum_in_list<'a>(
    list: impl Iterator<Item = &'a isize>,
    target: isize,
) -> Option<(isize, isize)> {
    let mut seen_set = HashSet::new();
    for &value in list {
        let difference = target - value;
        match seen_set.get(&difference) {
            None => seen_set.insert(value),
            Some(&v) => return Some((value, v)),
        };
    }

    None
}

pub fn a_optimal() {
    let input = include_str!("./inputs/day1.txt")
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    find_pair_of_numbers_with_sum_in_list(input.iter(), 2020);
}
