use crate::solution::Solution;


mod day01;

mod day02;

mod day03;

mod day04;

mod day05;

mod day06;

mod day07;

mod day08;


pub fn solve_day(day: u32) {
    match day {
        
        01 => day01::Day {}.solve(),
        
        02 => day02::Day {}.solve(),
        
        03 => day03::Day {}.solve(),
        
        04 => day04::Day {}.solve(),
        
        05 => day05::Day {}.solve(),
        
        06 => day06::Day {}.solve(),
        
        07 => day07::Day {}.solve(),
        
        08 => day08::Day {}.solve(),
        
        day => println!("{} has not been solved yet.", day),
    }
}
