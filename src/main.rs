use crate::solutions::solve_day;
use std::env;

mod solution;
mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();

    assert!(args.len() == 2);

    let day: u32 = args[1].parse().unwrap();

    solve_day(day);
}
