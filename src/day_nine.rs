use crate::day_one::find_pair_of_numbers_with_sum_in_list;
use std::error::Error;
use std::ops::Range;

#[derive(Debug, Default, Clone)]
struct XmasData {
    data_stream: Vec<isize>,
    preamble_size: usize,
}

impl XmasData {
    fn find_first_invalid_data_point(&self) -> Option<isize> {
        for (index, value) in self.data_stream[self.preamble_size..].iter().enumerate() {
            let offset = self.preamble_size + index;
            let validation_slice = &self.data_stream[index..offset];
            let valid_pair = find_pair_of_numbers_with_sum_in_list(validation_slice.iter(), *value);
            if valid_pair.is_none() {
                return Some(*value);
            }
        }

        None
    }

    fn find_contiguous_section_with_sum(&self, target: isize) -> Range<usize> {
        let mut selection = 0..1;
        loop {
            let sum = &self.data_stream[selection.clone()].iter().sum::<isize>();
            match sum {
                &x if x == target => {
                    return selection;
                }
                &x if x < target => {
                    selection.end += 1;
                }
                _ => {
                    selection.start += 1;
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_nine::XmasData;

    #[test]
    fn test_data_a() {
        let items = include_str!("inputs/day_nine.test.txt")
            .lines()
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        let mut xmas_data = XmasData {
            data_stream: items,
            preamble_size: 5,
        };

        assert_eq!(Some(127), xmas_data.find_first_invalid_data_point())
    }

    #[test]
    fn production_a() {
        let items = include_str!("inputs/day_nine.txt")
            .lines()
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        let mut xmas_data = XmasData {
            data_stream: items,
            preamble_size: 25,
        };

        assert_eq!(Some(375054920), xmas_data.find_first_invalid_data_point())
    }

    #[test]
    fn test_data_b() {
        let items = include_str!("inputs/day_nine.test.txt")
            .lines()
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        let mut xmas_data = XmasData {
            data_stream: items,
            preamble_size: 5,
        };
        let data_range = xmas_data.find_contiguous_section_with_sum(127);
        let range_set = &xmas_data.data_stream[data_range];

        assert_eq!(range_set.iter().min(), Some(&15));
        assert_eq!(range_set.iter().max(), Some(&47));
    }

    #[test]
    fn production_b() {
        let items = include_str!("inputs/day_nine.txt")
            .lines()
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        let mut xmas_data = XmasData {
            data_stream: items,
            preamble_size: 25,
        };
        let data_range = xmas_data.find_contiguous_section_with_sum(375054920);
        let range_set = &xmas_data.data_stream[data_range];

        assert_eq!(range_set.iter().min(), Some(&13369727));
        assert_eq!(range_set.iter().max(), Some(&40772857));
        // sum is 54142584
    }
}
