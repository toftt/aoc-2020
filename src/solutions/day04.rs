use crate::solution::Solution;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day;

trait Validate {
    fn validate(&self, s: &str) -> bool;
}

struct CustomValidator<F: Fn(&str) -> bool> {
    validate_fn: F,
}
impl<F: Fn(&str) -> bool> CustomValidator<F> {
    fn new(validate_fn: F) -> Self {
        CustomValidator { validate_fn }
    }
}
impl<F: Fn(&str) -> bool> Validate for CustomValidator<F> {
    fn validate(&self, s: &str) -> bool {
        (self.validate_fn)(s)
    }
}

struct BoundedValidator<T: PartialOrd + FromStr> {
    max: T,
    min: T,
}

impl<T> BoundedValidator<T>
where
    T: PartialOrd + FromStr,
{
    fn new(min: T, max: T) -> Self {
        BoundedValidator { min, max }
    }
}

impl<T> Validate for BoundedValidator<T>
where
    T: PartialOrd + FromStr,
{
    fn validate(&self, s: &str) -> bool {
        let parsed = T::from_str(s);

        match parsed {
            Ok(value) => value >= self.min && value <= self.max,
            Err(_) => false,
        }
    }
}

struct Field {
    name: String,
    validator: Box<dyn Validate>,
}

impl Field {
    fn new(name: &str, validator: impl Validate + 'static) -> Self {
        Field {
            name: String::from(name),
            validator: Box::new(validator),
        }
    }
}

impl Day {
    fn get_fields() -> Vec<Field> {
        vec![
            Field::new("byr", BoundedValidator::new(1920, 2002)),
            Field::new("iyr", BoundedValidator::new(2010, 2020)),
            Field::new("eyr", BoundedValidator::new(2020, 2030)),
            Field::new(
                "hgt",
                CustomValidator::new(|s| {
                    if s.len() < 3 {
                        return false;
                    }
                    let (value, unit) = s.split_at(s.len() - 2);
                    let (min, max) = match unit {
                        "cm" => (150, 193),
                        "in" => (59, 76),
                        _ => return false,
                    };

                    BoundedValidator::new(min, max).validate(value)
                }),
            ),
            Field::new(
                "hcl",
                CustomValidator::new(|s| {
                    if s.len() != 7 {
                        return false;
                    }

                    let without_hash = s.trim_start_matches("#");

                    if without_hash.len() != 6 {
                        return false;
                    }
                    without_hash.chars().all(|c| {
                        c.is_ascii_hexdigit() && (c.is_ascii_lowercase() || c.is_ascii_digit())
                    })
                }),
            ),
            Field::new(
                "ecl",
                CustomValidator::new(|s| {
                    let hair_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    hair_colors.contains(&s)
                }),
            ),
            Field::new(
                "pid",
                CustomValidator::new(|s| {
                    if s.len() != 9 {
                        return false;
                    }

                    s.chars().all(|c| c.is_ascii_digit())
                }),
            ),
        ]
    }
}

type Passport = HashMap<String, String>;
impl Solution for Day {
    type Input = Vec<Passport>;
    type Output1 = usize;
    type Output2 = usize;

    fn get_input_file_path(&self) -> String {
        "input/day04".to_string()
    }

    fn parse_input(&self, lines: Vec<String>) -> Vec<Passport> {
        let mut passports = vec![];
        let mut current = HashMap::new();

        for l in lines {
            for entry in l.split_ascii_whitespace() {
                let kv: Vec<_> = entry.split(':').collect();
                let (key, value) = (kv[0], kv[1]);
                current.insert(key.to_string(), value.to_string());
            }
            if l.len() == 0 {
                passports.push(current);
                current = HashMap::new();
            }
        }
        passports.push(current);

        passports
    }

    fn part1(&self, input: &Vec<Passport>) -> usize {
        let fields = Day::get_fields();

        input
            .iter()
            .map(|passport| {
                fields
                    .iter()
                    .all(|field| passport.contains_key(&field.name))
            })
            .filter(|b| *b)
            .count()
    }

    fn part2(&self, input: &Vec<Passport>) -> usize {
        let fields = Day::get_fields();
        let mut valid_count = 0;

        for passport in input {
            if fields.iter().all(|field| {
                passport
                    .get(&field.name)
                    .map_or(false, |value| field.validator.validate(value))
            }) {
                valid_count += 1;
            }
        }

        valid_count
    }
}
