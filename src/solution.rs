use std::{fmt::Display, fs};

pub trait Solution {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn get_input_file_path(&self) -> String;
    fn read_input_to_string(&self) -> String {
        let path = &self.get_input_file_path();

        fs::read_to_string(path).expect("no such file")
    }
    fn parse_input(&self, s: String) -> Self::Input;
    fn part1(&self, input: &Self::Input) -> Self::Output1;
    fn part2(&self, input: &Self::Input) -> Self::Output2;

    fn solve(&self) {
        let s = self.read_input_to_string();
        let input = self.parse_input(s);
        let solution1 = self.part1(&input);
        let solution2 = self.part2(&input);
        println!("Solution 1: {}", solution1);
        println!("Solution 2: {}", solution2);
    }
}
