use crate::solution::Solution;

pub struct Day;

fn modular_inverse(a: i64, n: i64) -> i64 {
    let mut t = 0;
    let mut new_t = 1;
    let mut r = n;
    let mut new_r = a;

    while new_r != 0 {
        let quotient = r / new_r;
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }

    if r > 1 {
        panic!("{} is not invertible in ring {}", a, n);
    }

    if t < 0 {
        t = t + n;
    }

    t
}

/// Solve the modular equation system using the chinese remainder theory.
///
/// Accepts two vectors of size k, where moduli[1..k] is coprime and greater than 1,
/// and remainders[1..k] are integers such that 0 <= remainders[i] <= moduli[i].
///
/// Returns one integer x, such that 0 <= x <= product(moduli) and the remainder
/// of the Euclidean divison of x by moduli[i] is remainders[i] for every i.
fn solve_mod_system(moduli: Vec<i64>, remainders: Vec<i64>) -> i64 {
    assert!(moduli.len() == remainders.len());

    let moduli_product: i64 = moduli.iter().product();

    let mut result = 0;
    for i in 0..moduli.len() {
        let n_i = moduli_product / moduli[i];

        result += remainders[i] * n_i * modular_inverse(n_i, moduli[i]);
    }

    return result % moduli_product;
}

impl Solution for Day {
    type Input = Vec<String>;
    type Output1 = i64;
    type Output2 = i64;

    fn get_input_file_path(&self) -> String {
        "input/day13".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> Self::Input {
        puzzle_input.lines().map(str::to_string).collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output1 {
        let timestamp: i64 = input[0].parse().unwrap();
        let bus_ids: Vec<i64> = input[1]
            .split(',')
            .filter(|s| *s != "x")
            .map(|s| s.parse().unwrap())
            .collect();

        let mut i = timestamp;
        loop {
            for bus_id in bus_ids.iter() {
                if i % bus_id == 0 {
                    return bus_id * (i - timestamp);
                }
            }
            i += 1;
        }
    }

    fn part2(&self, input: &Self::Input) -> Self::Output2 {
        let x: Vec<(i64, i64)> = input[1]
            .split(',')
            // the index in the array is the same as the "offset" that the bus needs to arrive at
            .enumerate()
            // we can filter out the 'x' entries after enumerating
            .filter(|(_offset, bus_id)| *bus_id != "x")
            .map(|(offset, bus_id)| (offset as i64, bus_id.parse::<i64>().unwrap()))
            // for the chinese theorem to work we need to transform equations such as `(x + 4) mod 16 = 0` into `x mod 16 = (0 - 4)`
            // we don't allow negative values so -4 becomes 12 in mod 16
            .map(|(offset, bus_id)| (bus_id - (offset % bus_id), bus_id))
            .collect();

        let moduli = x.iter().map(|(_offset, bus_id)| *bus_id).collect();
        let remainders = x.iter().map(|(offset, _bus_id)| *offset).collect();

        solve_mod_system(moduli, remainders)
    }
}
