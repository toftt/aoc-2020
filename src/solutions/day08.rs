use crate::solution::Solution;

use crate::utils;
use std::collections::HashMap;

pub struct Day;

fn check_terminates(instructions: &Vec<(String, i32)>) -> (bool, i32) {
    let mut acc = 0;
    let mut cur_idx = 0;

    let mut visited: Vec<usize> = Vec::new();

    while !visited.contains(&cur_idx) && cur_idx < instructions.len() {
        visited.push(cur_idx);

        let (ins, value) = &instructions[cur_idx];
        match &ins[..] {
            "nop" => cur_idx += 1,
            "acc" => {
                acc += value;
                cur_idx += 1
            }
            "jmp" => cur_idx = (cur_idx as i32 + value) as usize,
            _ => panic!("encountered invalid ins"),
        }
    }

    (cur_idx >= instructions.len(), acc)
}

type Instructions = Vec<(String, i32)>;
impl Solution for Day {
    type Input = Instructions;
    type Output1 = i32;
    type Output2 = i32;

    fn get_input_file_path(&self) -> String {
        "input/day08".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> Self::Input {
        puzzle_input
            .lines()
            .map(|l| utils::split::<String, i32>(l, " "))
            .collect()
    }

    fn part1(&self, input: &Instructions) -> Self::Output1 {
        check_terminates(&input).1
    }

    fn part2(&self, input: &Instructions) -> Self::Output2 {
        let mut replacements: HashMap<String, String> = HashMap::new();

        replacements.insert(String::from("nop"), String::from("jmp"));
        replacements.insert(String::from("jmp"), String::from("nop"));

        for (idx, line) in input.iter().enumerate() {
            let (ins, value) = line;

            if let Some(new_ins) = replacements.get(ins) {
                let mut new_vec = input.clone();
                new_vec[idx] = (new_ins.to_string(), *value);

                if let (true, acc) = check_terminates(&new_vec) {
                    return acc;
                }
            }
        }

        panic!("no result found!");
    }
}
