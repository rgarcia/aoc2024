use std::collections::BinaryHeap;
use std::collections::HashMap;
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

    // Start timing
    let start = std::time::Instant::now();

    match command.as_str() {
        "part1" => part1(file_path),
        "part2" => part2(file_path),
        _ => {
            eprintln!("Invalid command: {}. Use 'part1' or 'part2'.", command);
            std::process::exit(1);
        }
    }

    // Calculate and print elapsed time
    let duration = start.elapsed();
    eprintln!("Time: {:.3} seconds", duration.as_secs_f64());
}

fn part1(file_path: &str) {
    let file = File::open(file_path).expect("Something went wrong reading the file");
    let reader = io::BufReader::new(file);
    let mut heap1: BinaryHeap<i32> = BinaryHeap::new();
    let mut heap2: BinaryHeap<i32> = BinaryHeap::new();
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if line.trim().is_empty() {
            continue;
        }
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();
        if numbers.len() == 2 {
            heap1.push(numbers[0]);
            heap2.push(numbers[1]);
        } else {
            eprintln!("Invalid line: {}", line);
            std::process::exit(1);
        }
    }
    let mut sum = 0;
    while let (Some(v1), Some(v2)) = (heap1.pop(), heap2.pop()) {
        sum += (v2 - v1).abs();
    }

    println!("{}", sum);
}

fn part2(file_path: &str) {
    let file = File::open(file_path).expect("Something went wrong reading the file");
    let reader = io::BufReader::new(file);
    // First pass: count occurrences in second list
    let mut second_numbers: HashMap<i32, i32> = HashMap::new();
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if line.trim().is_empty() {
            continue;
        }
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();
        if numbers.len() == 2 {
            *second_numbers.entry(numbers[1]).or_insert(0) += 1;
        } else {
            eprintln!("Invalid line: {}", line);
            std::process::exit(1);
        }
    }

    // Second pass: calculate sum
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
        if numbers.len() == 2 {
            let first_num = numbers[0];
            let count = second_numbers.get(&first_num).unwrap_or(&0);
            sum += first_num * count;
        }
    }

    println!("{}", sum);
}
