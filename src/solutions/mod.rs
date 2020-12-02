use crate::solution::Solution;

mod day01;
mod day02;

pub fn solve_day(day: u32) {
    match day {
        01 => day01::Day {}.solve(),
        02 => day02::Day {}.solve(),
        day => println!("{} has not been solved yet.", day),
    }
}
