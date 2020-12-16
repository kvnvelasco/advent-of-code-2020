
use std::ops::Range;

#[derive(Debug)]
struct Finder(Range<u8>);

fn find_seat_row_number(source: &str) -> u8 {
    if source.len() != 7 {
        panic!("Exactly 7 sympols are required to find a seat row")
    }

    u8::from_str_radix(
        &source
            .chars()
            .map(|char| match char {
                'F' => '0',
                'B' => '1',
                _ => panic!("Invalid Command"),
            })
            .collect::<String>(),
        2,
    )
    .unwrap()
}

fn find_column_number(source: &str) -> u8 {
    if source.len() != 3 {
        panic!("Exactly 3 sympols are required to find a seat row")
    }

    u8::from_str_radix(
        &source
            .chars()
            .map(|char| match char {
                'L' => '0',
                'R' => '1',
                _ => panic!("Invalid Command"),
            })
            .collect::<String>(),
        2,
    )
    .unwrap()
}

fn process_boarding_pass(pass: &str) -> (u8, u8) {
    if pass.len() != 10 {
        panic!("A boarding pass is exactly 10 characters");
    }

    (
        find_seat_row_number(&pass[0..7]),
        find_column_number(&pass[7..10]),
    )
}

#[cfg(test)]
mod tests {
    use crate::day_five::{find_column_number, find_seat_row_number, process_boarding_pass};
    

    #[test]
    fn can_find_a_row() {
        assert_eq!(find_seat_row_number("FBFBBFF"), 44);
        assert_eq!(find_seat_row_number("BFFFBBF"), 70);
    }

    #[test]
    fn can_find_a_column() {
        assert_eq!(find_column_number("RRR"), 7);
        assert_eq!(find_column_number("RLL"), 4);
        assert_eq!(find_column_number("LLL"), 0);
    }

    #[test]
    fn it_can_find_a_boarding_pass_coordinate() {
        assert_eq!(process_boarding_pass("BFFFBBFRRR"), (70, 7));
        assert_eq!(process_boarding_pass("FFFBBBFRRR"), (14, 7));
    }

    #[test]
    fn production_run_a() {
        let boarding_passes = include_str!("inputs/day_five.txt").lines();
        let mut max = 0;

        for boarding_pass in boarding_passes {
            let (row, col) = process_boarding_pass(boarding_pass);
            max = max.max((row as usize * 8usize) + col as usize)
        }

        assert_eq!(max, 911)
    }

    #[test]
    fn production_run_b() {
        let all_passes = include_str!("inputs/day_five.txt")
            .lines()
            .map(process_boarding_pass)
            .map(|(row, col)| (row as usize * 8) + col as usize)
            .collect::<Vec<usize>>();

        // get our id by getting the theoretical maximum and subtracting the actual sum of all passes
        let maximum = all_passes.iter().max().unwrap();
        let minimum = all_passes.iter().min().unwrap();
        let sum_of_all_passes: usize = all_passes.iter().sum();

        let theoretical_maximum_sum_of_all_seats =
            (maximum.pow(2) - minimum.pow(2) - maximum - minimum) / 2;

        assert_eq!(theoretical_maximum_sum_of_all_seats - sum_of_all_passes, 9)
    }
}
