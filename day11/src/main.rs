use aochelpers::get_daily_input;
use std::{collections::HashMap, error::Error};

type Stone = u64;
type Count = usize;

const MULTIPLIER: Stone = 2024;
const PART1_STEPS: usize = 25;
const TOTAL_STEPS: usize = 75;

/// Represents the input data containing stones and their counts
struct Input {
    stones: HashMap<Stone, Count>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(11, 2024)?;

    let input = parse_data(&data);
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_data(input: &str) -> Input {
    let stones = input.split_whitespace().map(|n| n.parse().unwrap()).fold(
        HashMap::new(),
        |mut acc, num| {
            *acc.entry(num).or_default() += 1;
            acc
        },
    );
    Input { stones }
}

/// Transforms a single stone according to the rules
fn transform_stone(stone: Stone, count: Count, new_stones: &mut HashMap<Stone, Count>) {
    match stone {
        0 => *new_stones.entry(1).or_default() += count,
        n => {
            let digits = n.to_string();
            if digits.len() % 2 == 0 {
                let mid = digits.len() / 2;
                let left: Stone = digits[..mid].parse().unwrap();
                let right: Stone = digits[mid..].parse().unwrap();
                *new_stones.entry(left).or_default() += count;
                *new_stones.entry(right).or_default() += count;
            } else {
                *new_stones.entry(n * MULTIPLIER).or_default() += count;
            }
        }
    }
}

fn solve(input: &Input) -> (Count, Count) {
    let mut stones = input.stones.clone();
    let mut part1 = 0;

    for step in 0..TOTAL_STEPS {
        let mut new_stones = HashMap::new();

        for (&stone, &count) in &stones {
            transform_stone(stone, count, &mut new_stones);
        }
        stones = new_stones;

        if step == PART1_STEPS - 1 {
            part1 = stones.values().sum();
        }
    }

    (part1, stones.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "125 17";

    #[test]
    fn test_part1() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).0, 55312);
    }

    #[test]
    fn test_part2() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).1, 65601038650482);
    }
}
