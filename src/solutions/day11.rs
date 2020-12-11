use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    type Input = Vec<Vec<char>>;
    type Output1 = usize;
    type Output2 = usize;

    fn get_input_file_path(&self) -> String {
        "input/day11".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> Self::Input {
        puzzle_input
            .lines()
            .map(|l| l.chars().map(|c| c.to_owned()).collect())
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output1 {
        let directions = [
            (-1, -1),
            (1, 0),
            (0, 1),
            (1, 1),
            (-1, 0),
            (0, -1),
            (1, -1),
            (-1, 1),
        ] as [(i32, i32); 8];

        let mut grid = input.clone();

        loop {
            let mut changes = vec![];

            for (y_idx, line) in grid.iter().enumerate() {
                for (x_idx, token) in line.iter().enumerate() {
                    if *token == '.' {
                        continue;
                    }
                    let mut surrounding_tokens = vec![];

                    for (x, y) in directions.iter() {
                        let new_x = x_idx as i32 + x;
                        let new_y = y_idx as i32 + y;

                        if new_y >= 0
                            && new_y < grid.len() as i32
                            && new_x >= 0
                            && new_x < grid[0].len() as i32
                        {
                            surrounding_tokens.push(grid[new_y as usize][new_x as usize]);
                        } else {
                            surrounding_tokens.push('O');
                        }
                    }

                    match token {
                        '#' => {
                            if surrounding_tokens
                                .iter()
                                .filter(|c| **c == '#')
                                .collect::<Vec<&char>>()
                                .len()
                                >= 4
                            {
                                changes.push((x_idx, y_idx, 'L'));
                            }
                        }
                        'L' => {
                            if surrounding_tokens
                                .iter()
                                .filter(|c| **c == '#')
                                .collect::<Vec<&char>>()
                                .len()
                                == 0
                            {
                                changes.push((x_idx, y_idx, '#'));
                            }
                        }
                        '.' => (),
                        _ => panic!("got unexpected char"),
                    }
                }
            }
            //
            if changes.len() == 0 {
                break;
            }
            for (x, y, token) in changes {
                grid[y][x] = token;
            }
        }

        grid.concat()
            .iter()
            .filter(|c| **c == '#')
            .collect::<Vec<&char>>()
            .len()
    }

    fn part2(&self, input: &Self::Input) -> Self::Output2 {
        let directions = [
            (-1, -1),
            (1, 0),
            (0, 1),
            (1, 1),
            (-1, 0),
            (0, -1),
            (1, -1),
            (-1, 1),
        ] as [(i32, i32); 8];

        let mut grid = input.clone();

        loop {
            let mut changes = vec![];

            for (y_idx, line) in grid.iter().enumerate() {
                for (x_idx, token) in line.iter().enumerate() {
                    if *token == '.' {
                        continue;
                    }
                    let mut surrounding_tokens = vec![];

                    for (x, y) in directions.iter() {
                        let mut new_x = x_idx as i32 + x;
                        let mut new_y = y_idx as i32 + y;

                        while new_y >= 0
                            && new_y < grid.len() as i32
                            && new_x >= 0
                            && new_x < grid[0].len() as i32
                            && grid[new_y as usize][new_x as usize] != '#'
                            && grid[new_y as usize][new_x as usize] != 'L'
                        {
                            new_x = new_x + x;
                            new_y = new_y + y;
                        }

                        if new_y >= 0
                            && new_y < grid.len() as i32
                            && new_x >= 0
                            && new_x < grid[0].len() as i32
                        {
                            surrounding_tokens.push(grid[new_y as usize][new_x as usize]);
                        } else {
                            surrounding_tokens.push('O');
                        }
                    }

                    match token {
                        '#' => {
                            if surrounding_tokens
                                .iter()
                                .filter(|c| **c == '#')
                                .collect::<Vec<&char>>()
                                .len()
                                >= 5
                            {
                                changes.push((x_idx, y_idx, 'L'));
                            }
                        }
                        'L' => {
                            if surrounding_tokens
                                .iter()
                                .filter(|c| **c == '#')
                                .collect::<Vec<&char>>()
                                .len()
                                == 0
                            {
                                changes.push((x_idx, y_idx, '#'));
                            }
                        }
                        '.' => (),
                        _ => panic!("got unexpected char"),
                    }
                }
            }
            if changes.len() == 0 {
                break;
            }
            for (x, y, token) in changes {
                grid[y][x] = token;
            }
        }

        grid.concat()
            .iter()
            .filter(|c| **c == '#')
            .collect::<Vec<&char>>()
            .len()
    }
}
