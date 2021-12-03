use std::io::BufRead;

fn main() {
    println!("problem 1: {}", problem1());
    println!("problem 2: {}", problem2());
}

fn problem1() -> u32 {
    let file = std::fs::File::open("data/input.txt").unwrap();
    let mut lines = std::io::BufReader::new(file).lines();

    let mut num_increases: u32 = 0;
    let mut last_num: u32 = lines.next().unwrap().unwrap().parse().unwrap();

    for line in lines {
        if let Ok(num) = line.unwrap().parse::<u32>() {
            if num > last_num {
                num_increases += 1;
            }
            last_num = num;
        }
    }

    num_increases
}

fn problem2() -> u32 {
    let file = std::fs::File::open("data/input.txt").unwrap();
    let mut lines = std::io::BufReader::new(file).lines();

    let mut i = 0;
    let mut last_nums: Vec<u32> = lines
        .by_ref()
        .take(3)
        .map(|x| x.unwrap().parse().unwrap())
        .collect::<Vec<u32>>();

    let mut num_increases: u32 = 0;

    for line in lines {
        if let Ok(num) = line.unwrap().parse::<u32>() {
            if num > last_nums[i] {
                num_increases += 1;
            }
            last_nums[i] = num;
            i = (i + 1) % 3;
        }
    }

    num_increases
}
