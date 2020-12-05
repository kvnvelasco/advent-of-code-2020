use std::error::Error;
use std::io::BufRead;

#[derive(Eq, PartialEq, Debug)]
struct Line<'a> {
    min: usize,
    max: usize,
    character: char,
    input: &'a str,
}

impl<'a> Line<'a> {
    fn parse_from_str(input: &'a str) -> Result<Self, Box<dyn Error>> {
        let (spec, input) = {
            let mut iterator = input.split(':');
            (
                iterator.next().ok_or("No spec found")?,
                iterator.next().ok_or("No item found")?,
            )
        };

        let (min, max, character) = {
            let (minimum, max_with_input_set) = spec
                .find('-')
                .map(|path| (&spec[0..path], &spec[path + 1..]))
                .ok_or("Could not parse out spec")?;

            let mut whitespace_delimiter = max_with_input_set.split(' ');

            (
                minimum,
                whitespace_delimiter
                    .next()
                    .ok_or("could not find maximum")?,
                whitespace_delimiter
                    .next()
                    .ok_or("could not find character")?,
            )
        };

        Ok(Self {
            min: min.parse()?,
            max: max.parse()?,
            character: character.chars().next().ok_or("character does not exist")?,
            input: input.trim(),
        })
    }
    fn evaluate_valid_line_incorrectly(&self) -> bool {
        let mut character_count = 0;
        for character in self.input.chars() {
            if character == self.character {
                character_count += 1
            }
            if character_count > self.max {
                return false;
            }
        }
        character_count >= self.min
    }

    fn evaluate_valid_line_correctly(&self) -> bool {
        let character_bytes = self.input.as_bytes(); // we are certain that input can only contain ascii
        match (
            character_bytes.get(self.min - 1),
            character_bytes.get(self.max - 1),
        ) {
            (Some(&field), None) | (None, Some(&field)) if field == self.character as u8 => true,
            (Some(&left), Some(&right)) => {
                left != right && (left == self.character as u8 || right == self.character as u8)
            }
            _ => false,
        }
    }
}

pub fn a(mut input_set: impl Iterator<Item = &'static str>) -> Result<usize, Box<dyn Error>> {
    let lines = input_set.map(Line::parse_from_str);

    let mut count = 0;
    for line in lines {
        if line?.evaluate_valid_line_incorrectly() {
            count += 1
        };
    }
    Ok(count)
}

pub fn b(mut input_set: impl Iterator<Item = &'static str>) -> Result<usize, Box<dyn Error>> {
    let lines = input_set.map(Line::parse_from_str);

    let mut count = 0;
    for line in lines {
        if line?.evaluate_valid_line_correctly() {
            count += 1;
        };
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use crate::day_two::{a, b, Line};

    #[test]
    fn lines_are_parsable_from_strings() {
        let line = Line::parse_from_str("1-3 a: abcabc").unwrap();

        assert_eq!(
            line,
            Line {
                min: 1,
                max: 3,
                character: 'a',
                input: "abcabc"
            }
        )
    }

    #[test]
    fn valid_passwords_are_identified() {
        let input = include_str!("./inputs/day_two_test.txt").lines();
        let output = a(input).unwrap();

        assert_eq!(output, 2)
    }

    #[test]
    fn it_works_on_production_data() {
        let input = include_str!("./inputs/day_two.txt").lines();
        let output = a(input).unwrap();

        assert_eq!(output, 500)
    }

    #[test]
    fn correct_algorithm_detects_flaws_in_test_data() {
        let input = include_str!("./inputs/day_two_test.txt").lines();
        let output = b(input).unwrap();

        assert_eq!(output, 1)
    }

    #[test]
    fn correct_algorithm_works_on_production_data() {
        let input = include_str!("./inputs/day_two.txt").lines();
        let output = b(input).unwrap();

        assert_eq!(output, 313)
    }
}
