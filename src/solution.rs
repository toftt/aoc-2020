use std::fmt::Display;

pub trait Solution {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn load_input(&self) -> Self::Input;
    fn part1(&self, input: &Self::Input) -> Self::Output1;
    fn part2(&self, input: &Self::Input) -> Self::Output2;

    fn solve(&self) {
        let input = self.load_input();
        let solution1 = self.part1(&input);
        let solution2 = self.part2(&input);
        println!("Solution 1: {}", solution1);
        println!("Solution 2: {}", solution2);
    }
}
