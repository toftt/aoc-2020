use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn get_input_file_path(&self) -> String {
        "input/day01".to_string()
    }

    fn parse_input(&self, raw: String) -> Vec<i64> {
        raw.lines().flat_map(|line| line.parse()).collect()
    }

    fn part1(&self, input: &Vec<i64>) -> i64 {
        for i in 0..input.len() {
            for j in i..input.len() {
                let sum = input[i] + input[j];

                if sum == 2020 {
                    return input[i] * input[j];
                }
            }
        }
        panic!("no result found");
    }

    fn part2(&self, input: &Vec<i64>) -> i64 {
        for i in 0..input.len() {
            for j in i..input.len() {
                for k in j..input.len() {
                    let sum = input[i] + input[j] + input[k];

                    if sum == 2020 {
                        return input[i] * input[j] * input[k];
                    }
                }
            }
        }
        panic!("no result found");
    }
}
