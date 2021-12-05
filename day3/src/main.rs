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
    let oxygen_rating = problem2_helper(input, false);
    let co2_rating = problem2_helper(input, true);
    oxygen_rating * co2_rating
}

fn problem2_helper(input: &str, flip_bits: bool) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;
    let mut matrix = Matrix::new(lines);
    while matrix.num_rows() != 1 {
        let mut filter = matrix.most_common_bits();
        if flip_bits {
            filter = filter.flip_bits();
        };
        matrix = matrix.filter_rows(&filter, i);
        i += 1;
    }
    matrix.rows[0].to_u32()
}

#[derive(Debug)]
struct Row {
    bits: Vec<char>,
}

impl Row {
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

#[derive(Debug)]
struct Matrix {
    rows: Vec<Row>,
}

impl Matrix {
    fn new(lines: Vec<&str>) -> Self {
        Matrix {
            rows: lines.iter().map(|line| Row::new(line)).collect(),
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

    fn most_common_bits(&self) -> Row {
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
                if num_ones as f32 >= self.num_rows() as f32 / 2.0 {
                    '1'
                } else {
                    '0'
                }
            })
            .collect::<Vec<char>>();
        Row { bits }
    }

    fn filter_rows(&self, filter: &Row, col: usize) -> Self {
        let rows = self
            .rows
            .iter()
            .map(|row| row.bits.clone())
            .filter(|bits| bits[col] == filter.bits[col])
            .map(|bits| Row { bits })
            .collect();
        Self { rows }
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
