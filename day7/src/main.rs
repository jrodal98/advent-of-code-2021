fn main() {
    let input = include_str!("../data/input.txt");
    println!(
        "Problem 1: {}",
        solution(input, |crab_pos, target_pos| (crab_pos - target_pos).abs())
    );
    println!(
        "Problem 2: {}",
        solution(input, |crab_pos, target_pos| (((crab_pos - target_pos)
            .abs()
            * ((crab_pos - target_pos).abs() + 1))
            / 2))
    );
}

fn solution(input: &str, fuel_consumption_func: fn(i32, i32) -> i32) -> i32 {
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
                acc + fuel_consumption_func(crab_pos, target_pos)
            })
        })
        .min()
        .unwrap()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = solution(input, |crab_pos, target_pos| (crab_pos - target_pos).abs());
    assert_eq!(res, 37);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = solution(input, |crab_pos, target_pos| {
        (((crab_pos - target_pos).abs() * ((crab_pos - target_pos).abs() + 1)) / 2)
    });
    assert_eq!(res, 168);
}
