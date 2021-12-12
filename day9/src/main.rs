use std::arch::x86_64::_mm256_insertf128_ps;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u32 {
    Grid::new(input).solve_problem1()
}

fn problem2(input: &str) -> u32 {
    0
}

struct Location {
    value: u32,
    is_lowpoint: bool,
}

impl Location {
    fn new(c: char) -> Self {
        Self {
            value: c.to_digit(10).unwrap(),
            is_lowpoint: false,
        }
    }

    fn mark(&mut self) {
        self.is_lowpoint = true;
    }

    fn should_mark(&self, neighbors: Vec<&Location>) -> bool {
        self.value
            < neighbors
                .iter()
                .map(|location| location.value)
                .min()
                .unwrap()
    }
}

struct Grid {
    locations: Vec<Vec<Location>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let locations = input
            .lines()
            .map(|line| line.chars().map(Location::new).collect())
            .collect();
        Self { locations }
    }

    fn solve_problem1(&mut self) -> u32 {
        let last_row = self.locations.len() - 1;
        let last_col = self.locations[0].len() - 1;

        for row in 0..=last_row {
            for col in 0..=last_col {
                let mut neighbors = vec![];
                if row > 0 {
                    neighbors.push(&self.locations[row - 1][col]);
                }
                if row < last_row {
                    neighbors.push(&self.locations[row + 1][col]);
                }
                if col > 0 {
                    neighbors.push(&self.locations[row][col - 1]);
                }
                if col < last_col {
                    neighbors.push(&self.locations[row][col + 1]);
                }
                if self.locations[row][col].should_mark(neighbors) {
                    self.locations[row][col].mark()
                }
            }
        }

        self.locations
            .iter()
            .flatten()
            .filter(|location| location.is_lowpoint)
            .map(|location| location.value + 1)
            .sum()
    }
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 15);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 0);
}
