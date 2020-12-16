use crate::day_four::Field::{
    BirthYear, CountryID, ExpirationYear, EyeColor, HairColor, Height, IssueYear,
};
use std::collections::HashSet;


use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
enum Field<'a> {
    BirthYear(usize),
    IssueYear(usize),
    ExpirationYear(usize),
    Height(&'a str),
    HairColor(&'a str),
    EyeColor(&'a str),
    PID(&'a str),
    CountryID(&'a str),
}

impl<'a> Hash for Field<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_representation().hash(state)
    }
}

impl<'a> PartialEq for Field<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.type_representation() == other.type_representation()
    }
}

impl<'a> Field<'a> {
    fn from_string_field(source: &'static str) -> Option<Self> {
        let (name, value) = source.split_at(source.find(':')?);
        let value = &value[1..].trim();

        match name {
            "byr" => Some(BirthYear(value.parse().ok()?)),
            "iyr" => Some(IssueYear(value.parse().ok()?)),
            "eyr" => Some(ExpirationYear(value.parse().ok()?)),
            "hgt" => Some(Height(value)),
            "hcl" => Some(HairColor(value)),
            "ecl" => Some(EyeColor(value)),
            "pid" => Some(Field::PID(value)),
            "cid" => Some(CountryID(value)),
            _ => None,
        }
    }

    fn type_representation(&self) -> usize {
        match self {
            BirthYear(_) => 0,
            IssueYear(_) => 1,
            ExpirationYear(_) => 2,
            Height(_) => 3,
            HairColor(_) => 4,
            EyeColor(_) => 5,
            Self::PID(_) => 6,
            CountryID(_) => 7,
        }
    }

    fn validate(&self) -> bool {
        match *self {
            BirthYear(1920..=2002) => true,
            IssueYear(2010..=2020) => true,
            ExpirationYear(2020..=2030) => true,
            Height(h) => match (&h[..h.len() - 2].parse::<usize>(), &h[h.len() - 2..]) {
                (Ok(value @ 150..=193), unit @ "cm") | (Ok(value @ 59..=76), unit @ "in") => true,
                _ => false,
            },
            HairColor(color) => match (&color[0..1], &color[1..]) {
                ("#", hex) => u32::from_str_radix(hex, 16).is_ok(),
                _ => false,
            },
            EyeColor("amb") | EyeColor("blu") | EyeColor("brn") | EyeColor("gry")
            | EyeColor("grn") | EyeColor("hzl") | EyeColor("oth") => true,
            Self::PID(pid) => pid.chars().all(|c| c.is_numeric()) && pid.len() == 9,
            CountryID(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Default)]
struct Passport<'a> {
    fields: HashSet<Field<'a>>,
}

impl<'a> Passport<'a> {
    fn validate(&self) -> bool {
        let field_length = self.fields.len();
        if field_length < 7 {
            return false;
        };

        if field_length == 7 {
            !self.fields.contains(&Field::CountryID(""))
        } else {
            field_length == 8
        }
    }

    fn validate_with_field_validation(&self) -> bool {
        let field_length = self.fields.len();
        if field_length < 7 {
            return false;
        };

        let all_fields_valid = self.fields.iter().all(|x| x.validate());

        if field_length == 7 {
            !self.fields.contains(&Field::CountryID("")) && all_fields_valid
        } else {
            field_length == 8 && all_fields_valid
        }
    }
}

fn parse_passport_batch(source: &'static str) -> Vec<Passport> {
    let lines: Vec<&str> = source.lines().collect();
    let mut passports = vec![Default::default()];
    for line in lines {
        if line.is_empty() {
            passports.push(Passport::default());
            continue;
        }

        let current = passports.last_mut().unwrap();
        let fields = line.split_whitespace();

        for field in fields {
            let field = Field::from_string_field(field).expect("Unhandled field type");
            current.fields.insert(field);
        }
    }

    passports
}

#[cfg(test)]
mod tests {
    use crate::day_four::Field::{BirthYear, EyeColor, IssueYear};
    use crate::day_four::{parse_passport_batch, Field};
    use std::collections::HashSet;

    #[test]
    fn fields_are_parsable() {
        assert_eq!(Some(BirthYear(2002)), Field::from_string_field("byr:2002"));
        assert_eq!(Some(IssueYear(1992)), Field::from_string_field("iyr:1992"));
        assert_eq!(Some(EyeColor("gray")), Field::from_string_field("ecl:gray"))
    }

    #[test]
    fn field_hashes_ignore_values() {
        let mut set = HashSet::new();
        set.insert(BirthYear(2002));

        assert_eq!(set.get(&BirthYear(2000)), Some(&BirthYear(2002)))
    }

    #[test]
    fn parsing_batch_files() {
        let valid_count: usize = parse_passport_batch(include_str!("inputs/day_four.test.txt"))
            .iter()
            .map(|p| if p.validate() { 1 } else { 0 })
            .sum();

        assert_eq!(valid_count, 2)
    }

    #[test]
    fn parse_batch_files_prod() {
        let valid_count: usize = parse_passport_batch(include_str!("inputs/day_four.txt"))
            .iter()
            .map(|p| if p.validate() { 1 } else { 0 })
            .sum();

        assert_eq!(valid_count, 260)
    }

    #[test]
    fn test_field_validation() {
        let valid_count: usize =
            parse_passport_batch(include_str!("inputs/day_four_invalid.test.txt"))
                .iter()
                .map(|p| {
                    if p.validate_with_field_validation() {
                        1
                    } else {
                        0
                    }
                })
                .sum();

        assert_eq!(valid_count, 0);

        let valid_count: usize =
            parse_passport_batch(include_str!("inputs/day_four_valid.test.txt"))
                .iter()
                .map(|p| {
                    if p.validate_with_field_validation() {
                        1
                    } else {
                        0
                    }
                })
                .sum();

        assert_eq!(valid_count, 4);
    }

    #[test]
    fn parse_batch_files_prod_with_validation() {
        let valid_count: usize = parse_passport_batch(include_str!("inputs/day_four.txt"))
            .iter()
            .map(|p| {
                if p.validate_with_field_validation() {
                    1
                } else {
                    0
                }
            })
            .sum();

        assert_eq!(valid_count, 260)
    }
}
