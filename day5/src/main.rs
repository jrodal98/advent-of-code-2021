use std::collections::HashMap;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u32 {
    let graph = Graph::new(input);
    graph.calculate_score(true)
}

fn problem2(input: &str) -> u32 {
    let graph = Graph::new(input);
    graph.calculate_score(false)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    endpoint1: Coordinate,
    endpoint2: Coordinate,
}

#[derive(Debug)]
struct Graph {
    lines: Vec<Line>,
}

impl Coordinate {
    fn new(input: &str) -> Self {
        let (x_str, y_str) = input
            .split_once(',')
            .expect("Invalid format - expected x,y");
        let x = x_str.parse().expect("Invalid format: expected integer");
        let y = y_str.parse().expect("Invalid format: expected integer");
        Self { x, y }
    }
}

impl Line {
    fn new(input: &str) -> Self {
        let (endpoint1_str, endpoint2_str) = input
            .split_once(" -> ")
            .expect("Invalid format - expected x1,y1 -> x2,y2)");
        let endpoint1 = Coordinate::new(endpoint1_str);
        let endpoint2 = Coordinate::new(endpoint2_str);

        Self {
            endpoint1,
            endpoint2,
        }
    }
    fn is_vertical(&self) -> bool {
        self.endpoint1.x == self.endpoint2.x
    }

    fn is_horizontal(&self) -> bool {
        self.endpoint1.y == self.endpoint2.y
    }

    fn get_points(&self) -> Vec<Coordinate> {
        if self.is_horizontal() {
            let (min_endpoint, max_endpoint) = if self.endpoint1.x < self.endpoint2.x {
                (&self.endpoint1, &self.endpoint2)
            } else {
                (&self.endpoint2, &self.endpoint1)
            };

            (min_endpoint.x..max_endpoint.x + 1)
                .map(|x| Coordinate {
                    x,
                    y: min_endpoint.y,
                })
                .collect()
        } else if self.is_vertical() {
            let (min_endpoint, max_endpoint) = if self.endpoint1.y < self.endpoint2.y {
                (&self.endpoint1, &self.endpoint2)
            } else {
                (&self.endpoint2, &self.endpoint1)
            };

            (min_endpoint.y..max_endpoint.y + 1)
                .map(|y| Coordinate {
                    x: min_endpoint.x,
                    y,
                })
                .collect()
        } else {
            let (min_endpoint, max_endpoint) = if self.endpoint1.x < self.endpoint2.x {
                (&self.endpoint1, &self.endpoint2)
            } else {
                (&self.endpoint2, &self.endpoint1)
            };
            let delta_y = if min_endpoint.y < max_endpoint.y {
                1
            } else {
                -1
            };

            (0..(max_endpoint.x - min_endpoint.x + 1))
                .map(|x| Coordinate {
                    x: min_endpoint.x + x,
                    y: min_endpoint.y + x * delta_y,
                })
                .collect()
        }
    }
}

impl Graph {
    fn new(input: &str) -> Self {
        let lines: Vec<Line> = input.lines().map(Line::new).collect();
        Self { lines }
    }

    fn calculate_score(&self, straight_only: bool) -> u32 {
        self.get_valid_lines(straight_only)
            .iter()
            .fold(HashMap::new(), |mut acc, line| {
                for point in line.get_points() {
                    *acc.entry(point).or_insert(0) += 1;
                }
                acc
            })
            .values()
            .filter(|&&n| n > 1)
            .count()
            .try_into()
            .unwrap()
    }

    fn get_valid_lines(&self, straight_only: bool) -> Vec<&Line> {
        if straight_only {
            self.lines
                .iter()
                .filter(|line| line.is_vertical() || line.is_horizontal())
                .collect()
        } else {
            self.lines.iter().collect()
        }
    }
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 5);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 12);
}
