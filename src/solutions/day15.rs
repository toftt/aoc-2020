use crate::solution::Solution;
use std::collections::HashMap;

pub struct Day;

impl Solution for Day {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn get_input_file_path(&self) -> String {
        "input/day15".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> Self::Input {
        puzzle_input
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut seen: HashMap<i64, i64> = HashMap::new();

        for (idx, n) in input[0..input.len() - 1].iter().enumerate() {
            seen.insert(*n, (idx + 1) as i64);
        }

        let mut last = input[input.len() - 1];

        let start_turn = input.len() + 1;
        for turn in start_turn..2021 {
            if seen.contains_key(&last) {
                let diff = turn as i64 - 1 - seen.get(&last).unwrap();
                seen.insert(last, turn as i64 - 1);
                last = diff as i64;
            } else {
                seen.insert(last, turn as i64 - 1);
                last = 0;
            }
        }

        last
    }

    fn part2(&self, input: &Self::Input) -> Self::Output2 {
        let mut seen: HashMap<i64, i64> = HashMap::new();

        for (idx, n) in input[0..input.len() - 1].iter().enumerate() {
            seen.insert(*n, (idx + 1) as i64);
        }

        let mut last = input[input.len() - 1];

        let start_turn = input.len() + 1;
        for turn in start_turn..30_000_001 {
            if seen.contains_key(&last) {
                let diff = turn as i64 - 1 - seen.get(&last).unwrap();
                seen.insert(last, turn as i64 - 1);
                last = diff as i64;
            } else {
                seen.insert(last, turn as i64 - 1);
                last = 0;
            }
        }

        last
    }
}
