use std::env;
use std::fs::File;
use std::io::{self, BufRead};

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

fn is_safe(nums: &[i32]) -> bool {
    let all_increasing = nums.windows(2).all(|w| w[0] < w[1]);
    let all_decreasing = nums.windows(2).all(|w| w[0] > w[1]);
    let all_diff_one_to_three = nums
        .windows(2)
        .all(|w| (w[1] - w[0]).abs() >= 1 && (w[1] - w[0]).abs() <= 3);
    (all_increasing || all_decreasing) && all_diff_one_to_three
}

fn part1(file_path: &str) {
    let file = File::open(file_path).expect("Something went wrong reading the file");
    let reader = io::BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if line.trim().is_empty() {
            continue;
        }
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();
        if is_safe(&numbers) {
            sum += 1;
        }
    }
    println!("{}", sum);
}

fn part2(file_path: &str) {
    let file = File::open(file_path).expect("Something went wrong reading the file");
    let reader = io::BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if line.trim().is_empty() {
            continue;
        }
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();

        if is_safe(&numbers) {
            sum += 1;
        } else {
            for i in 0..numbers.len() {
                let mut modified_numbers = numbers.clone();
                modified_numbers.remove(i);
                if is_safe(&modified_numbers) {
                    sum += 1;
                    break;
                }
            }
        }
    }
    println!("{}", sum);
}
