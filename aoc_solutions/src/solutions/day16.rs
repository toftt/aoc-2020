use crate::assign_ints;
use crate::solution::Solution;
use crate::utils;

pub struct Day;

#[derive(Debug)]
pub struct Rule {
    field_name: String,
    range_1: (i64, i64),
    range_2: (i64, i64),
}

impl Rule {
    fn validate(&self, value: i64) -> bool {
        let result = value >= self.range_1.0 && value <= self.range_1.1
            || value >= self.range_2.0 && value <= self.range_2.1;
        println!(
            "testing {} for {:?}-{:?} == {}",
            value, self.range_1, self.range_2, result
        );

        result
    }
}

impl Solution for Day {
    type Input = (Vec<Rule>, Vec<i64>, Vec<Vec<i64>>);
    type Output1 = i64;
    type Output2 = i64;

    fn get_input_file_path(&self) -> String {
        "input/day16test".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> Self::Input {
        let sections: Vec<&str> = puzzle_input.split("\n\n").collect();
        let rules = sections[0]
            .lines()
            .map(|l| {
                let (name, rest): (String, String) = utils::split(l, ":");

                assign_ints!(&rest, r1_min, r1_max, r2_min, r2_max);

                Rule {
                    field_name: name,
                    range_1: (r1_min, r1_max),
                    range_2: (r2_min, r2_max),
                }
            })
            .collect();

        let mut nearby_tickets_iter = sections[2].lines();
        nearby_tickets_iter.advance_by(1).unwrap();

        let nearby_tickets = nearby_tickets_iter.map(|l| utils::get_ints(l)).collect();
        let my_ticket = utils::get_ints(sections[1].lines().nth(1).unwrap());

        (rules, my_ticket, nearby_tickets)
    }

    fn part1(&self, (rules, _my_ticket, nearby_tickets): &Self::Input) -> Self::Output1 {
        println!("{:?}", rules);
        nearby_tickets.concat().iter().fold(0, |acc, field| {
            if rules.iter().all(|rule| !rule.validate(*field)) {
                return acc + field;
            }
            acc
        })
    }

    fn part2(&self, _input: &Self::Input) -> Self::Output2 {
        4
    }
}
