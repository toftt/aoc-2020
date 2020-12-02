use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

pub trait Solution {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn get_input_file_path(&self) -> String;
    fn read_input_to_lines(&self) -> Vec<String> {
        let path = &self.get_input_file_path();
        let file = File::open(path).expect("no such file");
        let reader = BufReader::new(file);
        reader.lines().flatten().collect::<Vec<String>>()
    }
    fn parse_input(&self, lines: Vec<String>) -> Self::Input;
    fn part1(&self, input: &Self::Input) -> Self::Output1;
    fn part2(&self, input: &Self::Input) -> Self::Output2;

    fn solve(&self) {
        let lines = self.read_input_to_lines();
        let input = self.parse_input(lines);
        let solution1 = self.part1(&input);
        let solution2 = self.part2(&input);
        println!("Solution 1: {}", solution1);
        println!("Solution 2: {}", solution2);
    }
}
