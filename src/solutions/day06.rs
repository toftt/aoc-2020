use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    type Input = Vec<String>;
    type Output1 = i32;
    type Output2 = i32;

    fn get_input_file_path(&self) -> String {
        "input/day06".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> Vec<String> {
        puzzle_input.lines().map(str::to_string).collect()
    }

    fn part1(&self, input: &Vec<String>) -> i32 {
        1
    }

    fn part2(&self, input: &Vec<String>) -> i32 {
        1
    }
}
