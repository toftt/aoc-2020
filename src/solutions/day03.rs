use crate::solution::Solution;

pub struct Day;

fn count_trees_for_slope(input: &Vec<String>, slope: &(usize, usize)) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut trees = 0;

    while y < input.len() {
        x = (x + slope.0) % input[y].len();
        y += slope.1;

        if y >= input.len() {
            break;
        }

        if input[y].chars().nth(x).unwrap() == '#' {
            trees += 1;
        }
    }

    trees
}

impl Solution for Day {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn get_input_file_path(&self) -> String {
        "input/day03".to_string()
    }

    fn parse_input(&self, lines: Vec<String>) -> Vec<String> {
        lines
    }



    fn part1(&self, input: &Vec<String>) -> usize {
        count_trees_for_slope(input, &(3, 1))
    }

    fn part2(&self, input: &Vec<String>) -> usize {
        let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        slopes.iter().map(|slope| count_trees_for_slope(input, slope)).product::<usize>()
    }
}
