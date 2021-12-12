use std::collections::HashMap;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| Entry::new(line).output_values)
        .filter(|output_vec| match output_vec.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count()
}

fn problem2(input: &str) -> u32 {
    0
}

#[derive(Hash, Eq, PartialEq)]
struct Entry {
    signal_patterns: Vec<Vec<char>>,
    output_values: Vec<Vec<char>>,
}

impl Entry {
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

        Self {
            output_values: sections.pop().unwrap(),
            signal_patterns: sections.pop().unwrap(),
        }
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
    assert_eq!(res, 0);
}
