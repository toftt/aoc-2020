use crate::solutions::solve_day;
use handlebars::Handlebars;
use std::{
    env,
    collections::HashMap,
    fs
};

mod solution;
mod solutions;

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

fn get_days() -> Vec<String> {
    let paths = fs::read_dir("src/solutions").unwrap();
    let mut result = vec![];

    for path in paths {
        let path_string = path.unwrap().path().into_os_string().into_string().unwrap();
        result.push(path_string);
    }
    
    //let result = result.iter().filter(|p| p.contains("day")).collect();

    result
}

fn generate(day: u32) {
    println!("{:?}", get_days());
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("hello", include_str!("templates/mod.hbs"))
        .unwrap();
    
        let mut data = HashMap::new();
        data.insert("day", day);

    //println!("{}", handlebars.render("hello", &data).unwrap());
}