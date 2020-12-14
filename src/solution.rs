use std::{fmt::Display, fs, time::Instant};

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
        let start_solve = Instant::now();
        let solution1 = self.part1(&input);
        let end_solve_1 = Instant::now();
        let solution2 = self.part2(&input);
        let end_solve_2 = Instant::now();

        println!(
            "
Solve times
part 1:  {:?}
part 2:  {:?}
total:   {:?}
",
            end_solve_1.duration_since(start_solve),
            end_solve_2.duration_since(end_solve_1),
            end_solve_2.duration_since(start_solve)
        );

        println!("Answers");
        println!("Solution 1: {}", solution1);
        println!("Solution 2: {}", solution2);
    }
}
