use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    type Input = Vec<(char, i32)>;
    type Output1 = i32;
    type Output2 = i32;

    fn get_input_file_path(&self) -> String {
        "input/day12".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> Self::Input {
        puzzle_input
            .lines()
            .map(|l| {
                let spl = l.split_at(1);
                (spl.0.chars().nth(0).unwrap(), spl.1.parse().unwrap())
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut pos: (i32, i32) = (0, 0);
        let mut dir: i32 = 0;

        for (d, n) in input {
            match d {
                'N' => pos = (pos.0, pos.1 + n),
                'S' => pos = (pos.0, pos.1 - n),
                'E' => pos = (pos.0 + n, pos.1),
                'W' => pos = (pos.0 - n, pos.1),
                'R' => dir = (((dir + n) % 360) + 360) % 360,
                'L' => dir = (((dir - n) % 360) + 360) % 360,
                'F' => match dir {
                    0 => pos = (pos.0 + n, pos.1),
                    90 => pos = (pos.0, pos.1 - n),
                    180 => pos = (pos.0 - n, pos.1),
                    270 => pos = (pos.0, pos.1 + n),
                    _ => panic!(),
                },
                _ => panic!(),
            }
        }

        pos.0.abs() + pos.1.abs()
    }

    fn part2(&self, input: &Self::Input) -> Self::Output2 {
        let mut pos: (i32, i32) = (0, 0);
        let mut waypoint: (i32, i32) = (10, 1);

        for (d, n) in input {
            match d {
                'N' => waypoint = (waypoint.0, waypoint.1 + n),
                'S' => waypoint = (waypoint.0, waypoint.1 - n),
                'E' => waypoint = (waypoint.0 + n, waypoint.1),
                'W' => waypoint = (waypoint.0 - n, waypoint.1),
                'L' => match n {
                    90 => waypoint = (-waypoint.1, waypoint.0),
                    180 => waypoint = (-waypoint.0, -waypoint.1),
                    270 => waypoint = (waypoint.1, -waypoint.0),
                    _ => panic!(),
                },
                'R' => match n {
                    90 => waypoint = (waypoint.1, -waypoint.0),
                    180 => waypoint = (-waypoint.0, -waypoint.1),
                    270 => waypoint = (-waypoint.1, waypoint.0),
                    _ => panic!(),
                },
                'F' => pos = (pos.0 + waypoint.0 * n, pos.1 + waypoint.1 * n),
                _ => panic!(),
            }
        }

        pos.0.abs() + pos.1.abs()
    }
}
