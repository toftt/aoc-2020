use crate::solution::Solution;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

pub struct Day;

trait Point
where
    Self: Sized + Eq + Hash + Copy,
{
    fn get_neighboring_points(&self) -> Vec<Self>;
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn new(x: i64, y: i64, z: i64) -> Point3D {
        Point3D { x, y, z }
    }
}
impl Point for Point3D {
    fn get_neighboring_points(&self) -> Vec<Point3D> {
        (0..3)
            .map(|_| -1..2)
            .multi_cartesian_product()
            .filter(|coords| coords.iter().all(|coord| *coord == 0))
            .map(|coords| Point3D::new(self.x + coords[0], self.y + coords[1], self.z + coords[2]))
            .collect()
    }
}

#[allow(dead_code)]
struct PointGrid<T: Point> {
    grid: HashMap<T, bool>,
    pending_transformations: Vec<(T, bool)>,
}

impl<T: Point> PointGrid<T> {
    fn _new(points: Vec<(T, bool)>) -> Self {
        let mut grid = HashMap::new();
        for (point, active) in points {
            grid.insert(point, active);
        }

        PointGrid {
            grid,
            pending_transformations: vec![],
        }
    }

    fn _queue_transformations(&mut self, points: Vec<(&T, &bool)>) -> HashSet<T> {
        let mut neighbors_outside_current_grid = HashSet::new();

        for (point, active) in points.iter() {
            let neighbors = point.get_neighboring_points();

            let active_count = neighbors.iter().fold(0, |acc, p| {
                let neigbor_point = self.grid.get(p);

                if let None = neigbor_point {
                    neighbors_outside_current_grid.insert(**point);
                }

                if let Some(true) = neigbor_point {
                    return acc + 1;
                }
                acc
            });

            match (active, active_count) {
                (true, 2) | (true, 3) => {},
                (true, _) => { self.pending_transformations.push((**point, false)) },
                (false, 3) => { self.pending_transformations.push((**point, false)) },
                (false, _) => {},
            };
        }
        neighbors_outside_current_grid
    }
}

impl Solution for Day {
    type Input = HashMap<Point3D, bool>;
    type Output1 = i32;
    type Output2 = i32;

    fn get_input_file_path(&self) -> String {
        "input/day17".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> Self::Input {
        let mut grid = HashMap::new();
        for (y, line) in puzzle_input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.insert(
                    Point3D::new(x as i64, y as i64, 0),
                    match c {
                        '#' => true,
                        '.' => false,
                        other => panic!("Expect '#' or '.', got {}", other),
                    },
                );
            }
        }
        grid
    }

    fn part1(&self, grid: &Self::Input) -> Self::Output1 {
        for _ in 0..6 {
            let mut outskirt_cells = vec![];
            let mut transformations = vec![];

            for (point, active) in grid.iter() {
                let mut active_neighbor_count = 0;
                for neighbor in point.get_neighboring_points() {
                    if !grid.contains_key(&neighbor) {
                        outskirt_cells.push(neighbor);
                    } else if *grid.get(&neighbor).unwrap() == true {
                        active_neighbor_count += 1
                    }
                }

                if *active && !(active_neighbor_count == 2 || active_neighbor_count == 3) {
                    transformations.push((point, false));
                } else if !*active && active_neighbor_count == 3 {
                    transformations.push((point, true));
                }
            }
        }

        Point3D::new(0, 0, 0).get_neighboring_points();
        4
    }

    #[allow(unused_variables)]
    fn part2(&self, input: &Self::Input) -> Self::Output2 {
        4
    }
}
