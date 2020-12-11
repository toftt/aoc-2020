use std::{collections::HashSet, fmt, hash::Hash, ops::Sub, str::FromStr};

pub fn split<T, U>(s: &str, delimiter: &str) -> (T, U)
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
    U: FromStr,
    <U as FromStr>::Err: fmt::Debug,
{
    let mut parts = s.split(delimiter);
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

pub fn count_pairs_of_sum<T>(slice: &[T], sum: T) -> i32
where
    T: Eq + Sub<Output = T> + Hash + Copy,
{
    let mut set: HashSet<T> = HashSet::new();
    let mut count = 0;

    for num in slice {
        if set.get(&num).is_some() {
            count += 1;
        }

        set.insert(sum - *num);
    }

    count
}

struct CharGrid {
    grid: Vec<Vec<char>>,
}

impl CharGrid {
    fn new(s: &str) -> Self {
        CharGrid {
            grid: s
                .lines()
                .map(|l| l.chars().map(|c| c.to_owned()).collect::<Vec<char>>())
                .collect(),
        }
    }

    fn get_square(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }
}

impl fmt::Display for CharGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self
            .grid
            .iter()
            .fold(String::from(""), |acc, line| match acc.len() {
                0 => format!("{}", line.iter().collect::<String>()),
                _ => format!("{}\n{}", acc, line.iter().collect::<String>()),
            });

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::CharGrid;

    #[test]
    fn formatting() {
        let str_grid = "\
#.L
...
###";
        assert_eq!(format!("{}", CharGrid::new(str_grid)), str_grid)
    }
}
