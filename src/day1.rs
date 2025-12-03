enum Direction {
    Left,
    Right,
}
struct Instruction {
    direction: Direction,
    amount: u16,
}

fn parse_line(line: &str) -> Instruction {
    let mut chars = line.chars();
    let direction: Direction = match chars.next() {
        Some('R') => Direction::Right,
        Some('L') => Direction::Left,
        _ => panic!("unknown direction"),
    };
    let amount = chars.as_str().parse::<u16>().unwrap();

    return Instruction { direction, amount };
}

pub fn part1(input: String) {
    let lines = input.split("\n").filter(|x| *x != "");

    let mut dial: i16 = 50;
    let mut count: u32 = 0;
    for line in lines {
        let instruction = parse_line(line);
        match instruction.direction {
            Direction::Left => dial = (dial - instruction.amount as i16) % 100,
            Direction::Right => dial = (dial + instruction.amount as i16) % 100,
        }
        if dial == 0 {
            count += 1;
        }
        // println!(
        //     "{} {:>2} => new dial pos {:>3}",
        //     match instruction.direction {
        //         Direction::Left => "<",
        //         Direction::Right => ">",
        //     },
        //     instruction.amount,
        //     dial
        // );
    }

    println!("Result: {}", count);
}

pub fn part2(input: String) {
    let lines = input.split("\n").filter(|x| *x != "");

    let mut dial: i16 = 50;
    let mut count: u32 = 0;
    let mut prevDial: i16;
    for line in lines {
        prevDial = dial;
        let instruction = parse_line(line);
        match instruction.direction {
            Direction::Left => dial = dial - instruction.amount as i16,
            Direction::Right => dial = dial + instruction.amount as i16,
        }

        count += (dial / 100i16).abs() as u32;
        if prevDial != 0 && prevDial.signum() != dial.signum() {
            count += 1;
        }
        dial = dial.rem_euclid(100);

        println!(
            "{} {:>2} => new dial pos {:>3}  count: {:>6}",
            match instruction.direction {
                Direction::Left => "<",
                Direction::Right => ">",
            },
            instruction.amount,
            dial,
            count
        );
    }

    println!("Result: {}", count);
}
