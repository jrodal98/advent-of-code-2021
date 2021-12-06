use std::collections::HashMap;

fn main() {
    let input = include_str!("../data/input.txt");
    let (sol1, sol2) = solution(input);
    println!("Problem 1: {}", sol1);
    println!("Problem 2: {}", sol2);
}

fn solution(input: &str) -> (usize, usize) {
    let chips = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|raw_num| raw_num.parse::<usize>().unwrap())
        .collect();

    let finished_boards: Vec<Board> = input
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .chunks(5)
        .map(|board_slice| Board::new(board_slice))
        .map(|mut board| {
            let _ = &board.play_game(&chips);
            board
        })
        .filter(|board| board.final_chip.is_some())
        .collect();

    let first_winning_board = finished_boards
        .iter()
        .min_by_key(|board| board.num_chips_played)
        .unwrap();
    let last_winning_board = finished_boards
        .iter()
        .max_by_key(|board| board.num_chips_played)
        .unwrap();

    (
        first_winning_board.calculate_score(),
        last_winning_board.calculate_score(),
    )
}

struct Board {
    chips: HashMap<usize, (usize, usize)>,
    rows: Vec<usize>,
    cols: Vec<usize>,
    num_chips_played: usize,
    final_chip: Option<usize>,
}

impl Board {
    fn new(lines: &[&str]) -> Self {
        let chips = lines.iter().enumerate().fold(
            HashMap::new(),
            |mut acc: HashMap<usize, (usize, usize)>, (row, &line)| {
                line.split_whitespace()
                    .enumerate()
                    .map(|(col, raw_num)| acc.insert(raw_num.parse::<usize>().unwrap(), (row, col)))
                    .count();
                acc
            },
        );
        Board {
            chips,
            rows: vec![0; 5],
            cols: vec![0; 5],
            num_chips_played: 0,
            final_chip: None,
        }
    }

    fn place_chip(&mut self, chip: usize) -> bool {
        self.num_chips_played += 1;
        if let Some((row, col)) = self.chips.remove(&chip) {
            self.rows[row] += 1;
            self.cols[col] += 1;
        }
        // if game is over
        self.rows
            .iter()
            .any(|&chip_placed_in_row| chip_placed_in_row == 5)
            || self
                .cols
                .iter()
                .any(|&chip_placed_in_row| chip_placed_in_row == 5)
    }

    fn play_game(&mut self, chips: &Vec<usize>) {
        for chip in chips {
            if self.place_chip(*chip) {
                self.final_chip = Some(*chip);
                return;
            }
        }
    }

    fn calculate_score(&self) -> usize {
        let sum_of_unmarked: usize = self.chips.keys().sum();
        sum_of_unmarked * self.final_chip.unwrap_or(0)
    }
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let (res, _) = solution(input);
    assert_eq!(res, 4512);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let (_, res) = solution(input);
    assert_eq!(res, 1924);
}
