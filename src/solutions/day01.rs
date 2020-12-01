use crate::solution::Solution;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Day;

impl Solution for Day {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn load_input(&self) -> Vec<i64> {
        let file = File::open("input/day01").expect("no such file");
        let reader = BufReader::new(file);
        reader.lines().flatten().flat_map(|l| l.parse()).collect()
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
