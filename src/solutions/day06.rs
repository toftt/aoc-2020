use crate::solution::Solution;
use std::collections::HashMap;

pub struct Day;

impl Solution for Day {
    type Input = Vec<Vec<String>>;
    type Output1 = usize;
    type Output2 = usize;

    fn get_input_file_path(&self) -> String {
        "input/day06".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> Vec<Vec<String>> {
        puzzle_input
            .split("\n\n")
            .map(|group| group.split("\n").map(str::to_string).collect())
            .collect()
    }

    fn part1(&self, input: &Vec<Vec<String>>) -> usize {
        input
            .iter()
            .map(|group| {
                let mut questions: HashMap<char, u32> = HashMap::new();

                for answers in group {
                    for answer in answers.chars() {
                        let count = questions.get(&answer).map_or(1, |val| val + 1);
                        questions.insert(answer, count);
                    }
                }
                questions.keys().len()
            })
            .sum()
    }

    fn part2(&self, input: &Vec<Vec<String>>) -> usize {
        input
            .iter()
            .map(|group| {
                let mut questions: HashMap<char, usize> = HashMap::new();

                for answers in group {
                    for answer in answers.chars() {
                        let count = questions.get(&answer).map_or(1, |val| val + 1);
                        questions.insert(answer, count);
                    }
                }
                questions
                    .values()
                    .filter(|v| **v == group.len())
                    .collect::<Vec<_>>()
                    .len()
            })
            .sum()
    }
}
