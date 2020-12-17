#![feature(iterator_fold_self)]
#![feature(destructuring_assignment)]
#![feature(iter_advance_by)]

use crate::solutions::solve_day;
use handlebars::Handlebars;
use std::{collections::HashMap, env, fs};

mod solution;
mod solutions;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    assert!(args.len() == 3);

    let task = &args[1];
    let day: u32 = args[2].parse().unwrap();

    match task.as_str() {
        "solve" => solve(day),
        "generate" => generate(day),
        _ => panic!("valid arguments are 'solve' or 'generate'"),
    }
}

fn solve(day: u32) {
    solve_day(day);
}

fn get_days_with_solution() -> Vec<String> {
    let paths = fs::read_dir("src/solutions").unwrap();
    let mut result = vec![];

    for path in paths {
        let path_string = path.unwrap().path().into_os_string().into_string().unwrap();
        result.push(path_string);
    }
    let mut days: Vec<u32> = result
        .into_iter()
        .filter(|p| p.contains("day"))
        .map(|day_path| {
            day_path
                .trim_end_matches(".rs")
                .split("/")
                .last()
                .unwrap()
                .trim_start_matches("day")
                .parse()
                .unwrap()
        })
        .collect();

    days.sort();
    days.iter().map(|d| format!("{:02}", d)).collect()
}

fn get_days_with_input() -> Vec<String> {
    let paths = fs::read_dir("input").unwrap();
    let mut result = vec![];

    for path in paths {
        let path_string = path.unwrap().path().into_os_string().into_string().unwrap();
        result.push(path_string);
    }
    let mut days: Vec<u32> = result
        .into_iter()
        .map(|day_path| {
            day_path
                .split("/")
                .last()
                .unwrap()
                .trim_start_matches("day")
                .parse()
                .unwrap()
        })
        .collect();

    days.sort();
    days.iter().map(|d| format!("{:02}", d)).collect()
}

fn generate(day: u32) {
    let day = format!("{:02}", day);
    let days_with_solution = get_days_with_solution();
    let days_with_input = get_days_with_input();

    if !days_with_input.contains(&day) {
        println!("Creating empty input file for day{}", day);
        fs::File::create(format!("input/day{}", day)).unwrap();
    } else {
        println!("Input file for day{} already exists, skipping", day);
    }

    let mut handlebars = Handlebars::new();

    handlebars.set_strict_mode(true);
    handlebars
        .register_template_string("mod.rs", include_str!("templates/mod.hbs"))
        .unwrap();
    handlebars
        .register_template_string("day.rs", include_str!("templates/day.hbs"))
        .unwrap();

    if !days_with_solution.contains(&day) {
        println!("Creating solution file for day{}", day);

        let file = fs::File::create(format!("src/solutions/day{}.rs", day)).unwrap();
        let mut data = HashMap::new();
        data.insert("day", day);

        handlebars.render_to_write("day.rs", &data, file).unwrap();
    } else {
        println!("Solution file for day{} already exists, skipping", day);
    }

    println!("Modifying 'mod.rs' to include all solution files");

    let file = fs::File::create("src/solutions/mod.rs").unwrap();
    let mut data = HashMap::new();

    data.insert("day", get_days_with_solution());
    handlebars.render_to_write("mod.rs", &data, file).unwrap();
}
