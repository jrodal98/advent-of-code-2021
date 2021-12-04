fn main() {
    let input = include_str!("../data/input.txt");
    let final_coordinate = Coordinate::from_input_file(input);
    println!(
        "Problem 1 Solution: {}\nProblem 2 Solution: {}",
        final_coordinate.problem1(),
        final_coordinate.problem2()
    )
}

#[derive(Default)]
struct Coordinate {
    horizontal: usize,
    depth: usize,
    aim: usize,
}

impl Coordinate {
    fn from_input_file(input: &str) -> Self {
        input
            .lines()
            .map(|line| Instruction::from_line(line).unwrap())
            .fold(Coordinate::default(), |mut acc, instruction| {
                acc.move_submarine(instruction);
                acc
            })
    }
    fn move_submarine(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Forward(val) => {
                self.horizontal += val;
                self.depth += val * self.aim;
            }
            Instruction::Down(val) => self.aim += val,
            Instruction::Up(val) => self.aim -= val,
        }
    }

    fn problem1(&self) -> usize {
        self.horizontal * self.aim
    }
    fn problem2(&self) -> usize {
        self.horizontal * self.depth
    }
}

enum Instruction {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl Instruction {
    fn from_line(line: &str) -> Option<Instruction> {
        match line.split_once(' ').unwrap() {
            ("forward", val) => Some(Instruction::Forward(val.parse().unwrap())),
            ("down", val) => Some(Instruction::Down(val.parse().unwrap())),
            ("up", val) => Some(Instruction::Up(val.parse().unwrap())),
            _ => None,
        }
    }
}

#[test]
fn test_problem1_sample() {
    let input = include_str!("../data/sample.txt");
    let pos = Coordinate::from_input_file(input);
    assert_eq!(pos.problem1(), 150);
}

#[test]
fn test_problem2_sample() {
    let input = include_str!("../data/sample.txt");
    let pos = Coordinate::from_input_file(input);
    assert_eq!(pos.problem2(), 900);
}
