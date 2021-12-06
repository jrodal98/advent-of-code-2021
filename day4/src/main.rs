const NUM_ROWS: usize = 5;
const NUM_COLS: usize = 5;
const MAX_VAL: usize = 100; // real bingo only has 75, but this input doesn't for some reason...

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
        .map(Board::new)
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

#[derive(Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
    is_marked: bool,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            is_marked: false,
        }
    }

    fn mark(&mut self) {
        self.is_marked = true;
    }
}

struct Board {
    chips: [Option<Coordinate>; MAX_VAL],
    n_marked_chips_per_row: [usize; NUM_ROWS],
    n_marked_chips_per_col: [usize; NUM_COLS],
    num_chips_played: usize,
    final_chip: Option<usize>,
}

impl Board {
    fn new(lines: &[&str]) -> Self {
        let chips = lines
            .iter()
            .enumerate()
            .fold([None; MAX_VAL], |mut acc, (row, &line)| {
                line.split_whitespace()
                    .enumerate()
                    .map(|(col, raw_num)| {
                        acc[raw_num.parse::<usize>().unwrap()] = Some(Coordinate::new(row, col))
                    })
                    .count();
                acc
            });
        Board {
            chips,
            n_marked_chips_per_row: [0; 5],
            n_marked_chips_per_col: [0; 5],
            num_chips_played: 0,
            final_chip: None,
        }
    }

    fn place_chip(&mut self, chip: usize) {
        self.num_chips_played += 1;
        if let Some(coordinate) = &mut self.chips[chip] {
            self.n_marked_chips_per_row[coordinate.x] += 1;
            self.n_marked_chips_per_col[coordinate.y] += 1;
            coordinate.mark();
        }
    }

    fn is_board_complete(&self) -> bool {
        self.n_marked_chips_per_row
            .iter()
            .any(|&chip_placed_in_row| chip_placed_in_row == 5)
            || self
                .n_marked_chips_per_col
                .iter()
                .any(|&chip_placed_in_row| chip_placed_in_row == 5)
    }

    fn play_game(&mut self, chips: &Vec<usize>) {
        for chip in chips {
            self.place_chip(*chip);
            if self.is_board_complete() {
                self.final_chip = Some(*chip);
                return;
            }
        }
    }

    fn calculate_score(&self) -> usize {
        let sum_of_unmarked: usize = self
            .chips
            .iter()
            .enumerate()
            .map(|(i, coordinate)| {
                if let Some(c) = coordinate {
                    if c.is_marked {
                        0
                    } else {
                        i
                    }
                } else {
                    0
                }
            })
            .sum();
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
