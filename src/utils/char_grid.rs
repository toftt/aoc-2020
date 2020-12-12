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

    fn get_square(&self, x: usize, y: usize) -> &char {
        &self.grid[y][x]
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
