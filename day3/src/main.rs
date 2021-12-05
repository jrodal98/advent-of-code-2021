fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let matrix = Matrix::new(lines);
    let gamma_bits = matrix.most_common_bits();
    let epsilon_bits = &gamma_bits.flip_bits();
    gamma_bits.to_u32() * epsilon_bits.to_u32()
}

fn problem2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let matrix = Matrix::new(lines);
    let gamma_bits = matrix.most_common_bits();
    let epsilon_bits = &gamma_bits.flip_bits();
    gamma_bits.to_u32() * epsilon_bits.to_u32()
}

#[derive(Debug)]
struct Matrix {
    rows: Vec<BitRow>,
}

#[derive(Debug)]
struct BitRow {
    bits: Vec<char>,
}

impl BitRow {
    fn new(line: &str) -> Self {
        Self {
            bits: line.chars().collect(),
        }
    }

    fn flip_bits(&self) -> Self {
        let bits = self
            .bits
            .iter()
            .map(|c| match c {
                '1' => '0',
                '0' => '1',
                _ => unreachable!(),
            })
            .collect();
        Self { bits }
    }
    fn to_u32(&self) -> u32 {
        u32::from_str_radix(&self.bits.iter().collect::<String>(), 2).unwrap()
    }
}

impl Matrix {
    fn new(lines: Vec<&str>) -> Self {
        Matrix {
            rows: lines.iter().map(|line| BitRow::new(line)).collect(),
        }
    }

    fn num_rows(&self) -> usize {
        self.rows.len()
    }

    fn num_cols(&self) -> usize {
        if self.num_rows() > 0 {
            self.rows[0].bits.len()
        } else {
            0
        }
    }

    fn most_common_bits(&self) -> BitRow {
        let bits = self
            .rows
            .iter()
            .fold(vec![0; self.num_cols()], |mut acc, row| {
                for (i, val) in row.bits.iter().enumerate() {
                    acc[i] += val.to_digit(10).unwrap();
                }
                acc
            })
            .iter()
            .map(|&num_ones| {
                if num_ones >= (self.num_rows() / 2).try_into().unwrap() {
                    '1'
                } else {
                    '0'
                }
            })
            .collect::<Vec<char>>();
        BitRow { bits }
    }
}

#[derive(Debug)]
struct Game {
    vals: Vec<i32>,
}

fn to_u32(bit_vec: &Vec<char>) -> u32 {
    u32::from_str_radix(&bit_vec.iter().collect::<String>(), 2).unwrap()
}

fn get_gamma_bits(vals: &[i32]) -> Vec<char> {
    vals.iter()
        .map(|&v| if v >= 0 { '1' } else { '0' })
        .collect::<Vec<char>>()
}

fn get_epsilon_bits(vals: &[i32]) -> Vec<char> {
    vals.iter()
        .map(|&v| if v < 0 { '1' } else { '0' })
        .collect::<Vec<char>>()
}

impl Game {
    fn init_from_line(line: &str) -> Self {
        Self {
            vals: line
                .chars()
                .map(|c| match c {
                    '0' => -1,
                    '1' => 1,
                    _ => unreachable!(),
                })
                .collect::<Vec<i32>>(),
        }
    }

    fn add_row(&mut self, other: &Self) {
        for (i, val) in other.vals.iter().enumerate() {
            self.vals[i] += val;
        }
    }

    fn compute_problem1(&self) -> u32 {
        let gamma_bits = get_gamma_bits(&self.vals);
        let epsilon_bits = get_epsilon_bits(&self.vals);

        let gamma = to_u32(&gamma_bits);
        let epsilon = to_u32(&epsilon_bits);

        gamma * epsilon
    }

    fn compute_problem2(&self, input: &str) -> u32 {
        let gamma_bits = get_gamma_bits(&self.vals);
        let mut lines: Vec<&str> = input.lines().collect();

        let mut i = 0;
        while lines.len() != 1 {
            lines = lines
                .into_iter()
                .filter(|line| line.chars().nth(i).unwrap() == gamma_bits[i])
                .collect::<Vec<&str>>();
            i += 1;
        }
        dbg!(lines);
        0
    }
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 198);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 230);
}
