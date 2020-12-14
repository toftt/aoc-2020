use crate::solution::Solution;
use std::collections::HashMap;

pub struct Day;

pub enum Instruction {
    SetMask { mask: String },
    SetMemory { address: i64, value: i64 },
}

use Instruction::{SetMask, SetMemory};
impl Solution for Day {
    type Input = Vec<Instruction>;
    type Output1 = i64;
    type Output2 = i64;

    fn get_input_file_path(&self) -> String {
        "input/day14".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> Self::Input {
        puzzle_input
            .lines()
            .map(|l| {
                let parts: Vec<&str> = l.split_ascii_whitespace().collect();

                match parts[0] {
                    "mask" => SetMask {
                        mask: parts[2].to_string(),
                    },
                    _ => SetMemory {
                        address: parts[0][4..parts[0].len() - 1].parse().unwrap(),
                        value: parts[2].parse().unwrap(),
                    },
                }
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut current_mask: Option<&str> = None;
        let mut memory: HashMap<i64, i64> = HashMap::new();

        for instruction in input {
            match instruction {
                SetMask { mask } => current_mask = Some(mask),
                SetMemory { address, value } => {
                    let masked_value = current_mask.unwrap().chars().rev().enumerate().fold(
                        *value,
                        |acc, (idx, c)| match c {
                            '1' => acc | (1 << idx),
                            '0' => acc & !(1 << idx),
                            _ => acc,
                        },
                    );

                    memory.insert(*address, masked_value);
                }
            }
        }

        memory.values().sum()
    }

    fn part2(&self, input: &Self::Input) -> Self::Output2 {
        let mut current_mask: Option<&str> = None;
        let mut memory: HashMap<i64, i64> = HashMap::new();

        for instruction in input {
            match instruction {
                SetMask { mask } => current_mask = Some(mask),
                SetMemory { address, value } => {
                    for m in generate_all_masks(current_mask.unwrap()) {
                        let masked_address =
                            m.chars()
                                .rev()
                                .enumerate()
                                .fold(*address, |acc, (idx, c)| match c {
                                    '1' | 'I' => acc | (1 << idx),
                                    '0' => acc,
                                    'O' => acc & !(1 << idx),
                                    _ => acc,
                                });

                        memory.insert(masked_address, *value);
                    }
                }
            }
        }

        memory.values().sum()
    }
}

fn generate_all_masks(mask: &str) -> Vec<String> {
    if mask.matches('X').count() == 0 {
        return vec![mask.to_string()];
    }

    let mut p1 = generate_all_masks(&mask.replacen('X', "I", 1));
    let mut p2 = generate_all_masks(&mask.replacen('X', "O", 1));

    p1.append(&mut p2);
    p1
}

#[test]
fn generate() {
    let result = generate_all_masks("0X1XX1");
    println!("{:?}", result);
}
