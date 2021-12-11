use std::collections::HashMap;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", solution(input, 80));
    println!("Problem 2: {}", solution(input, 256));
}

fn solution(input: &str, num_days: u64) -> u64 {
    let mut cache = HashMap::new();
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .fold(0, |acc, s| {
            acc + get_num_children(s, num_days, &mut cache) + 1
        })
}

fn get_num_children(state: u64, days_remaining: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if days_remaining == 0 {
        return 0;
    }
    let cache_key = &(state, days_remaining);
    if let Some(cached_val) = cache.get(cache_key) {
        return *cached_val;
    }

    let num_children = if state == 0 {
        1 + get_num_children(6, days_remaining - 1, cache)
            + get_num_children(8, days_remaining - 1, cache)
    } else {
        get_num_children(state - 1, days_remaining - 1, cache)
    };

    if cache.insert(*cache_key, num_children).is_some() {
        panic!("If this happens, then clowntown");
    }
    num_children
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = solution(input, 80);
    assert_eq!(res, 5934);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = solution(input, 256);
    assert_eq!(res, 26984457539);
}
