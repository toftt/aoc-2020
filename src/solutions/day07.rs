use crate::solution::Solution;
use std::collections::HashMap;

pub struct Day;

type BagRecord = HashMap<String, HashMap<String, u32>>;

fn get_count(record: &BagRecord, name: &str) -> u32 {
    if record.get(name).is_none() {
        return 1;
    }
    let mut count = 0;
    for (name, inner_bag_count) in record.get(name).unwrap() {
        let x = get_count(record, name);
        match x {
            1 => count += inner_bag_count,
            _ => count += x * inner_bag_count + inner_bag_count,
        };
    }

    return count;
}

impl Solution for Day {
    type Input = BagRecord;
    type Output1 = i32;
    type Output2 = u32;

    fn get_input_file_path(&self) -> String {
        "input/day07".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> BagRecord {
        let mut result = HashMap::new();

        for l in puzzle_input.lines() {
            let mut contents = HashMap::new();

            let f: Vec<&str> = l.split("contain").collect();
            let name = f[0].trim().trim_end_matches("s");

            for bag in f[1].trim_end_matches(".").split(',') {
                let bag = bag.trim();
                if bag == "no other bags" {
                    break;
                }

                let amount: u32 = bag.chars().nth(0).unwrap().to_digit(10).unwrap();
                let inner_name = bag[1..].trim().trim_end_matches("s");

                contents.insert(inner_name.to_string(), amount);
            }
            if contents.len() != 0 {
                result.insert(name.to_string(), contents);
            }
        }
        result
    }

    fn part1(&self, input: &BagRecord) -> i32 {
        let mut count = 0;
        for (name, contents) in input {
            if name == "shiny gold bag" {
                continue;
            }

            let mut contains_shiny = false;
            let mut list_of_keys: Vec<&str> = contents.keys().map(|k| k.as_str()).collect();

            while list_of_keys.len() != 0 {
                let current = list_of_keys.pop().unwrap();

                if current == "shiny gold bag" {
                    contains_shiny = true;
                    break;
                }

                input.get(current).map(|map| {
                    for key in map.keys() {
                        list_of_keys.push(key);
                    }
                });
            }
            if contains_shiny {
                count += 1;
            }
        }

        count
    }

    fn part2(&self, input: &BagRecord) -> u32 {
        return get_count(input, "shiny gold bag");
    }
}
