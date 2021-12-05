fn main() {
    let input = include_str!("../data/input.txt");
    println!("problem 1: {}", solution(input, 2));
    println!("problem 2: {}", solution(input, 4));
}

fn solution(input: &str, window_size: usize) -> usize {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>()
        .windows(window_size)
        .filter(|window| window[window_size - 1] > window[0])
        .count()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = solution(input, 2);
    assert_eq!(res, 7);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = solution(input, 4);
    assert_eq!(res, 5);
}
