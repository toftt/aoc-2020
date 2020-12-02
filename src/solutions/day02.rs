use crate::solution::Solution;

pub struct PasswordPolicy {
    low: u32,
    high: u32,
    letter: char,
    password: String,
}

pub struct Day;

impl Solution for Day {
    type Input = Vec<PasswordPolicy>;
    type Output1 = u64;
    type Output2 = u64;

    fn get_input_file_path(&self) -> String {
        "input/day02".to_string()
    }

    fn parse_input(&self, lines: Vec<String>) -> Vec<PasswordPolicy> {
        lines.iter().map(|l| {
            let mut iter = l.split_ascii_whitespace();
            let mut high_low = iter.next().unwrap().split('-');

            let low: u32 = high_low.next().unwrap().parse().unwrap();
            let high: u32 = high_low.next().unwrap().parse().unwrap();

            let letter = iter.next().unwrap().chars().next().unwrap();
            let password = iter.next().unwrap().to_string();

            PasswordPolicy {
                low,
                high,
                letter,
                password
            }
        }).collect()
    }

    fn part1(&self, input: &Vec<PasswordPolicy>) -> u64 {
        let mut valid_password_count = 0;

        for policy in input {
            let occurences = policy.password.matches(policy.letter).count();
            if occurences <= policy.high as usize && occurences >= policy.low as usize {
                valid_password_count += 1;
            }
        }

        valid_password_count
    }

    fn part2(&self, input: &Vec<PasswordPolicy>) -> u64 {
        let mut valid_password_count = 0;

        for policy in input {
            let mut occurences = 0;

            if policy.password.chars().nth((policy.low as usize) - 1) == Some(policy.letter) {
                occurences += 1;
            }
            if policy.password.chars().nth((policy.high as usize) - 1) == Some(policy.letter) {
                occurences += 1;
            }

            if occurences == 1 {
                valid_password_count += 1;
            }
        }

        valid_password_count
    }
}
