fn main() {
    let input = include_str!("../data/input.txt");
    let crab_data = &CrabData::new(input);
    let problem1 = Problem1 { crab_data };
    let problem2 = Problem2 { crab_data };
    println!("Problem 1: {}", problem1.compute_min_fuel_use());
    println!("Problem 2: {}", problem2.compute_min_fuel_use());
}

struct CrabData {
    crab_positions: Vec<i32>,
    min_crab: i32,
    max_crab: i32,
}

impl CrabData {
    fn new(input: &str) -> Self {
        let crab_positions: Vec<i32> = input
            .trim()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();

        let &min_crab = crab_positions.iter().min().unwrap();
        let &max_crab = crab_positions.iter().max().unwrap();

        Self {
            crab_positions,
            min_crab,
            max_crab,
        }
    }
}

trait CrabSubmarine {
    fn consume_fuel(crab_pos: i32, target_pos: i32) -> i32;
    fn get_crab_data(&self) -> &CrabData;

    fn compute_min_fuel_use(&self) -> i32 {
        let crab_data = self.get_crab_data();

        (crab_data.min_crab..=crab_data.max_crab)
            .map(|target_pos| {
                crab_data.crab_positions.iter().fold(0, |acc, &crab_pos| {
                    acc + Self::consume_fuel(crab_pos, target_pos)
                })
            })
            .min()
            .unwrap()
    }
}

struct Problem1<'a> {
    crab_data: &'a CrabData,
}

impl CrabSubmarine for Problem1<'_> {
    fn consume_fuel(crab_pos: i32, target_pos: i32) -> i32 {
        (crab_pos - target_pos).abs()
    }

    fn get_crab_data(&self) -> &CrabData {
        self.crab_data
    }
}

struct Problem2<'a> {
    crab_data: &'a CrabData,
}

impl CrabSubmarine for Problem2<'_> {
    fn consume_fuel(crab_pos: i32, target_pos: i32) -> i32 {
        ((crab_pos - target_pos).abs() * ((crab_pos - target_pos).abs() + 1)) / 2
    }
    fn get_crab_data(&self) -> &CrabData {
        self.crab_data
    }
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let crab_data = &CrabData::new(input);
    let problem1 = Problem1 { crab_data };
    let res = problem1.compute_min_fuel_use();
    assert_eq!(res, 37);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let crab_data = &CrabData::new(input);
    let problem2 = Problem2 { crab_data };
    let res = problem2.compute_min_fuel_use();
    assert_eq!(res, 168);
}
