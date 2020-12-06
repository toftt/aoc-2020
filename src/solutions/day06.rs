use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    type Input = Vec<i32>;
    type Output1 = i32;
    type Output2 = i32;

    fn get_input_file_path(&self) -> String {
        "input/day06".to_string()
    }

    fn parse_input(&self, lines: Vec<String>) -> Vec<i32> {
        lines.iter().flat_map(|line| line.parse()).collect()
    }

    fn part1(&self, input: &Vec<i32>) -> i32 {
        1
    }

    fn part2(&self, input: &Vec<i32>) -> i32 {
        1
    }
}

