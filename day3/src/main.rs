
fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
}

fn problem1(input: &str) -> u32 {
    let nums: Game = input
        .lines()
        .map(Game::init_from_line)
        .reduce(|mut acc, next| {
            acc.add_row(&next);
            acc
        })
        .unwrap();
    nums.compute_problem1()
}

#[derive(Debug)]
struct Game {
    vals: Vec<i32>,
}

fn to_u32(slice: &[u8]) -> u32 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}

impl Game {
    fn init_from_line(line: &str) -> Self {
        Self {
            vals: line
                .chars()
                .map(|c| match c {
                    '0' => -1,
                    '1' => 1,
                    _ => unreachable!(),
                })
                .collect::<Vec<i32>>(),
        }
    }

    fn add_row(&mut self, other: &Self) {
        for (i, val) in other.vals.iter().enumerate() {
            self.vals[i] += val;
        }
    }

    fn compute_problem1(&self) -> u32 {
        let gamma_bits = self
            .vals
            .iter()
            .map(|&v| (v > 0) as u8)
            .collect::<Vec<u8>>();

        let epsilon_bits = self
            .vals
            .iter()
            .map(|&v| (v < 0) as u8)
            .collect::<Vec<u8>>();


        let gamma = to_u32(&gamma_bits);
        let epsilon = to_u32(&epsilon_bits);

        gamma * epsilon
    }
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 198);
}
