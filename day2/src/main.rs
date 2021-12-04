fn main() {
    let input = include_str!("../data/input.txt");
    dbg!(solution(input));
}

fn solution(input: &str) -> (usize, usize) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (dir, val) in get_instructions(input) {
        match dir {
            Direction::Forward => {
                horizontal += val;
                depth += val * aim;
            }
            Direction::Down => aim += val,
            Direction::Up => aim -= val,
        }
    }
    (horizontal * aim, horizontal * depth)
}


fn get_instructions<'a>(input: &'a str) -> impl Iterator<Item = (Direction, usize)> + 'a {
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(dir, val)| {
            (
                Direction::from_str(dir).unwrap(),
                val.parse::<usize>().unwrap(),
            )
        })
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
fn test_solution() {
    let input = include_str!("../data/sample.txt");
    let (problem1, problem2) = solution(input);
    assert_eq!(problem1, 150);
    assert_eq!(problem2, 900);
}
