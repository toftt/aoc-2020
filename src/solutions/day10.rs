use crate::solution::Solution;
use graphlib::Graph;
use graphlib::VertexId;
use std::collections::HashMap;

pub struct Day;

impl Solution for Day {
    type Input = Vec<i32>;
    type Output1 = usize;
    type Output2 = u64;

    fn get_input_file_path(&self) -> String {
        "input/day10".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> Self::Input {
        puzzle_input.lines().map(|l| l.parse().unwrap()).collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut new_inp = input.clone();
        new_inp.push(*new_inp.iter().max().unwrap() + 3);
        new_inp.push(0);
        new_inp.sort();

        let diffs: Vec<i32> = new_inp.windows(2).map(|w| w[1] - w[0]).collect();

        (diffs
            .iter()
            .filter(|d| **d == 1)
            .map(|f| f.to_string())
            .collect::<String>()
            .len())
            * (diffs
                .iter()
                .filter(|d| **d == 3)
                .map(|f| f.to_string())
                .collect::<String>()
                .len())
    }

    fn part2(&self, input: &Self::Input) -> Self::Output2 {
        let mut graph: Graph<i32> = Graph::new();

        let mut verts = vec![];
        let start = graph.add_vertex(0);
        let end = graph.add_vertex(*input.iter().max().unwrap() + 3);

        verts.push(start);
        verts.push(end);
        for outlet in input {
            verts.push(graph.add_vertex(*outlet));
        }

        for i in 0..verts.len() {
            for j in 0..verts.len() {
                let val1 = *graph.fetch(&verts[i]).unwrap();
                let val2 = *graph.fetch(&verts[j]).unwrap();

                if val1 > val2 && val1 - val2 <= 3 {
                    graph.add_edge(&verts[j], &verts[i]).unwrap();
                }
            }
        }

        count_paths(&graph, &end)
    }
}

fn count_paths(g: &Graph<i32>, dest: &VertexId) -> u64 {
    let mut counts: HashMap<&VertexId, u64> = HashMap::new();

    for vertex in g.topo() {
        let in_neighbors: Vec<&VertexId> = g.in_neighbors(vertex).collect();

        if in_neighbors.len() == 0 {
            counts.insert(vertex, 1);
        } else {
            let mut acc = 0;
            for neighbor in in_neighbors {
                acc += counts.get(neighbor).unwrap();
            }
            counts.insert(vertex, acc as u64);
        }
    }

    *counts.get(dest).unwrap()
}
