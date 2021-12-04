fn main() {
    println!("{}", problem1());
}

fn problem1() -> u32 {
    let input = include_str!("../data/sample.txt");
    let coordinates: Vec<Vec<&str>> = input
        .lines()
        .map(|line| {
            line.split_whitespace().collect()
        })
        .collect();

    let mut x = 0;
    let mut y = 0;
    for coordinate in coordinates {
        let (direction, val) = match &coordinate[..] {
            &[first, second, ..] => (first, second.parse::<u32>().unwrap()),
            _ => unreachable!(),
        };
        match direction {
            "forward" => x += val,
            "down"=> y += val,
            "up" => y -= val,
            _  => panic!("Improperly formatted input")
        };
    }
    return x * y;
}
