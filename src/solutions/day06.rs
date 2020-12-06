use crate::solution::Solution;
use std::collections::HashSet;
use std::iter::FromIterator;

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
                group
                    .into_iter()
                    .map(|ans| HashSet::from_iter(ans.chars()))
                    .fold_first(|acc: HashSet<char>, set| acc.union(&set).cloned().collect())
                    .unwrap()
                    .len()
            })
            .sum()
    }

    fn part2(&self, input: &Vec<Vec<String>>) -> usize {
        input
            .iter()
            .map(|group| {
                group
                    .into_iter()
                    .map(|ans| HashSet::from_iter(ans.chars()))
                    .fold_first(|acc: HashSet<char>, set| acc.intersection(&set).cloned().collect())
                    .unwrap()
                    .len()
            })
            .sum()
    }
}
