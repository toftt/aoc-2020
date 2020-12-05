use crate::solution::Solution;

pub struct Day;



impl Solution for Day {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn get_input_file_path(&self) -> String {
        "input/day04".to_string()
    }

    fn parse_input(&self, lines: Vec<String>) -> Vec<String> {
        lines
    }



    fn part1(&self, input: &Vec<String>) -> usize {
        4
    }

    fn part2(&self, input: &Vec<String>) -> usize {
        4
    }
}
