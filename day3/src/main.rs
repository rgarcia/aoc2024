use nom::{
    branch::alt, bytes::complete::tag, character::complete::u64 as nom_u64, combinator::map,
    sequence::tuple, IResult,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <part1|part2> <file_path>", args[0]);
        std::process::exit(1);
    }

    let command = &args[1];
    let file_path = &args[2];

    let start = std::time::Instant::now();
    match command.as_str() {
        "part1" => part1(file_path),
        "part2" => part2(file_path),
        _ => {
            eprintln!("Invalid command: {}. Use 'part1' or 'part2'.", command);
            std::process::exit(1);
        }
    }
    let duration = start.elapsed();
    eprintln!("Time: {:.6} seconds", duration.as_secs_f64());
}

enum ParseResult {
    Do,
    Dont,
    Mul(u64, u64),
}

fn parse_next(s: &str) -> IResult<&str, ParseResult> {
    alt((
        map(tag("do()"), |_| ParseResult::Do),
        map(tag("don't()"), |_| ParseResult::Dont),
        map(
            tuple((tag("mul("), nom_u64, tag(","), nom_u64, tag(")"))),
            |(_, a, _, b, _)| ParseResult::Mul(a, b),
        ),
    ))(s)
}

fn part1(file_path: &str) {
    let input = std::fs::read_to_string(file_path).expect("Failed to read file");
    let sum = parse(input, false);
    println!("{}", sum);
}

fn parse(input: String, do_enabled: bool) -> u64 {
    let mut input = input.as_str();
    let mut sum = 0;
    let mut enabled = true;

    while !input.is_empty() {
        let Ok((rem, parsed)) = parse_next(input) else {
            input = &input[1..];
            continue;
        };
        input = rem;
        match parsed {
            ParseResult::Do => enabled = true,
            ParseResult::Dont => enabled = false,
            ParseResult::Mul(a, b) => {
                if !do_enabled || enabled {
                    sum += a * b;
                }
            }
        }
    }
    return sum;
}

fn part2(file_path: &str) {
    let input = std::fs::read_to_string(file_path).expect("Failed to read file");
    let sum = parse(input, true);
    println!("{}", sum);
}
