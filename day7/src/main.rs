fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", Problem1::compute_min_fuel_use(input));
    println!("Problem 2: {}", Problem2::compute_min_fuel_use(input));
}

struct Problem1;
struct Problem2;

trait CrabSubmarine {
    fn consume_fuel(crab_pos: i32, target_pos: i32) -> i32;

    fn compute_min_fuel_use(input: &str) -> i32 {
        let positions: Vec<i32> = input
            .trim()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();

        let &min_pos = positions.iter().min().unwrap();
        let &max_pos = positions.iter().max().unwrap();

        (min_pos..=max_pos)
            .map(|target_pos| {
                positions.iter().fold(0, |acc, &crab_pos| {
                    acc + Self::consume_fuel(crab_pos, target_pos)
                })
            })
            .min()
            .unwrap()
    }
}

impl CrabSubmarine for Problem1 {
    fn consume_fuel(crab_pos: i32, target_pos: i32) -> i32 {
        (crab_pos - target_pos).abs()
    }
}

impl CrabSubmarine for Problem2 {
    fn consume_fuel(crab_pos: i32, target_pos: i32) -> i32 {
        ((crab_pos - target_pos).abs() * ((crab_pos - target_pos).abs() + 1)) / 2
    }
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = Problem1::compute_min_fuel_use(input);
    assert_eq!(res, 37);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = Problem2::compute_min_fuel_use(input);
    assert_eq!(res, 168);
}
