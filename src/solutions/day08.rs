use crate::solution::Solution;

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

    fn parse_input(&self, puzzle_input: String) -> Instructions {
        puzzle_input
            .lines()
            .map(|l| l.split_ascii_whitespace().collect::<Vec<&str>>())
            .map(|v| (v[0].to_string(), v[1].parse::<i32>().unwrap()))
            .collect()
    }

    fn part1(&self, input: &Instructions) -> i32 {
        check_terminates(&input).1
    }

    fn part2(&self, input: &Instructions) -> i32 {
        // let transformations = [("nop", "jmp"), ("jmp", "nop")];
        let mut eventual_acc = 9;

        for (idx, line) in input.iter().enumerate() {
            let (ins, value) = line;

            if &ins[..] == "nop" {
                let mut new_vec = input.clone();
                new_vec[idx] = ("jmp".to_string(), *value);
                let (term, ac) = check_terminates(&new_vec);

                if term {
                    eventual_acc = ac;
                    break;
                }
            }
            if &ins[..] == "jmp" {
                let mut new_vec = input.clone();
                new_vec[idx] = ("nop".to_string(), *value);
                let (term, ac) = check_terminates(&new_vec);

                if term {
                    eventual_acc = ac;
                    break;
                }
            }
        }

        eventual_acc
    }
}
