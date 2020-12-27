use crate::solution::Solution;
use crate::utils;

pub struct Day;

impl Solution for Day {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn get_input_file_path(&self) -> String {
        "input/day09".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> Self::Input {
        puzzle_input.lines().map(|l| l.parse().unwrap()).collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output1 {
        for nums in input.windows(26) {
            let last_elem_idx = nums.len() - 1;

            if utils::count_pairs_of_sum(&nums[..last_elem_idx], nums[last_elem_idx]) == 0 {
                return nums[last_elem_idx];
            }
        }

        panic!("did not find solution");
    }

    fn part2(&self, input: &Self::Input) -> Self::Output2 {
        let number_to_find = self.part1(input);

        let mut window_size = 2;

        while window_size <= input.len() {
            for nums in input.windows(window_size) {
                let sum: i64 = nums.iter().sum();
                if number_to_find == sum {
                    return nums.iter().max().unwrap() + nums.iter().min().unwrap();
                }
            }
            window_size += 1;
        }
        panic!("did not find solution");
    }
}
