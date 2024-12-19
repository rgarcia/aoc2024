use aochelpers::get_daily_input;
use std::{collections::HashMap, error::Error};

#[derive(Debug)]
struct Input {
    patterns: Vec<String>,
    designs: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(19, 2024)?;
    let input = parse_data(&data);
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

fn parse_data(input: &str) -> Input {
    let mut parts = input.split("\n\n");
    let patterns = parts
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let designs = parts
        .next()
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    Input { patterns, designs }
}

fn count_ways_to_make_design(patterns: &[String], design: &str) -> usize {
    fn dfs(patterns: &[String], remaining: &str, memo: &mut HashMap<String, usize>) -> usize {
        if remaining.is_empty() {
            return 1;
        }
        if let Some(&result) = memo.get(remaining) {
            return result;
        }
        let mut total_ways = 0;
        for pattern in patterns {
            if remaining.starts_with(pattern) {
                total_ways += dfs(patterns, &remaining[pattern.len()..], memo);
            }
        }
        memo.insert(remaining.to_string(), total_ways);
        total_ways
    }

    let mut memo = HashMap::new();
    dfs(patterns, design, &mut memo)
}

fn solve(input: &Input) -> (usize, usize) {
    let possible_designs: Vec<usize> = input
        .designs
        .iter()
        .map(|design| count_ways_to_make_design(&input.patterns, design))
        .filter(|ways| *ways > 0)
        .collect();
    (possible_designs.len(), possible_designs.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part1() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).0, 6);
    }

    #[test]
    fn test_part2() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).1, 16);
    }
}
