use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    type Input = Vec<String>;
    type Output1 = u32;
    type Output2 = u32;

    fn get_input_file_path(&self) -> String {
        "input/day05".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> Vec<String> {
        puzzle_input.lines().map(str::to_string).collect()
    }

    fn part1(&self, input: &Vec<String>) -> u32 {
        input
            .iter()
            .map(|l| {
                l.replace(|c| c == 'F' || c == 'L', "0")
                    .replace(|c| c == 'B' || c == 'R', "1")
            })
            .flat_map(|n| u32::from_str_radix(&n, 2))
            .max()
            .unwrap()
    }

    fn part2(&self, input: &Vec<String>) -> u32 {
        let mut seat_ids = input
            .iter()
            .map(|l| {
                l.replace(|c| c == 'F' || c == 'L', "0")
                    .replace(|c| c == 'B' || c == 'R', "1")
            })
            .flat_map(|n| u32::from_str_radix(&n, 2))
            .collect::<Vec<u32>>();

        seat_ids.sort();
        seat_ids.windows(2).find(|x| x[1] != x[0] + 1).unwrap()[0] + 1
    }
}
