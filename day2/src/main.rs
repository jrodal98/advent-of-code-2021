fn main() {
    let input = include_str!("../data/input.txt");
    println!("{}", problem1(input));
    println!("{}", problem2(input));
}

fn problem1(input: &str) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;
    for (direction, val) in get_instructions(input) {
        match direction {
            Direction::Forward => horizontal += val,
            Direction::Down => depth += val,
            Direction::Up => depth -= val,
        };
    }
    return horizontal * depth;
}

fn problem2(input: &str) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (direction, val) in get_instructions(input) {
        match direction {
            Direction::Forward => {
                horizontal += val;
                depth += aim * val;
            },
            Direction::Down => aim += val,
            Direction::Up => aim -= val,
        };
    }
    return horizontal * depth;
}

fn get_instructions(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .map(|line| {
            let raw_instructions: Vec<&str> = line.split_whitespace().collect();
            match &raw_instructions[..] {
                &[direction, val, ..] => (
                    Direction::from_str(direction).expect("Invalid direction, check input"),
                    val.parse::<usize>().expect("Could not parse value into unsigned int"),
                ),
                _ => unreachable!(),
            }
        })
        .collect()
}

enum Direction {
    Forward,
    Up,
    Down,
}

impl Direction {
    fn from_str(input: &str) -> Option<Direction> {
        match input {
            "forward" => Some(Direction::Forward),
            "down" => Some(Direction::Down),
            "up" => Some(Direction::Up),
            _ => None,
        }
    }
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 150);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 900);
}
