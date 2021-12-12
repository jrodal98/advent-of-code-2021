fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| Solver::new(line).output_values)
        .filter(|output_vec| match output_vec.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count()
}

fn problem2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Solver::new(line).compute_output_val())
        .sum()
}

struct Translator {
    one_chars: Vec<char>,
    four_chars: Vec<char>,
}

impl Translator {
    fn new(jumbled_wires: &Vec<Vec<char>>) -> Self {
        let one_chars = jumbled_wires
            .iter()
            .find(|wire| wire.len() == 2)
            .unwrap()
            .to_vec();
        let four_chars = jumbled_wires
            .iter()
            .find(|wire| wire.len() == 4)
            .unwrap()
            .to_vec();

        Self {
            one_chars,
            four_chars,
        }
    }

    fn num_common_with_one(&self, jumbled_wire: &Vec<char>) -> u8 {
        Self::num_common_segments(&self.one_chars, jumbled_wire)
    }

    fn num_common_with_four(&self, jumbled_wire: &Vec<char>) -> u8 {
        Self::num_common_segments(&self.four_chars, jumbled_wire)
    }

    fn num_common_segments(wire1: &Vec<char>, wire2: &Vec<char>) -> u8 {
        wire1
            .iter()
            .fold(0, |acc, c| if wire2.contains(c) { acc + 1 } else { acc })
    }

    fn translate(&self, jumbled_wire: &Vec<char>) -> char {
        match (
            jumbled_wire.len(),
            self.num_common_with_one(jumbled_wire),
            self.num_common_with_four(jumbled_wire),
        ) {
            (2, _, _) => '1',
            (5, _, 2) => '2',
            (5, 2, _) => '3',
            (4, _, _) => '4',
            (5, 1, _) => '5',
            (6, 1, _) => '6',
            (3, _, _) => '7',
            (7, _, _) => '8',
            (6, _, 4) => '9',
            (6, _, 3) => '0',
            (_, _, _) => unreachable!(),
        }
    }
}

struct Solver {
    output_values: Vec<Vec<char>>,
    translator: Translator,
}

impl Solver {
    fn new(input: &str) -> Self {
        let mut sections = input
            .split(" | ")
            .map(|section| {
                section
                    .split_whitespace()
                    .map(|p| p.chars().collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let output_values = sections.pop().unwrap();
        let signal_patterns = sections.pop().unwrap();
        let translator = Translator::new(&signal_patterns);

        Self {
            output_values,
            translator,
        }
    }

    fn compute_output_val(&self) -> u32 {
        self.output_values
            .iter()
            .map(|wire| self.translator.translate(wire))
            .collect::<String>()
            .parse::<u32>()
            .unwrap()
    }
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 26);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 61229);
}
