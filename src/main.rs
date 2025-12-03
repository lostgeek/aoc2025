mod day1;

fn main() {
    let day = std::env::args().nth(1).expect("no day given");
    let part = std::env::args().nth(2).expect("no part given");
    let input_file = std::env::args().nth(3).expect("no input file given");

    let input = std::fs::read_to_string(input_file).expect("failed to read input file");

    match day.as_str() {
        "1" => match part.as_str() {
            "1" => day1::part1(input),
            "2" => day1::part2(input),
            _ => panic!("unknown part"),
        },
        _ => panic!("unknown day"),
    }
}
