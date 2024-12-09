use aochelpers::{get_daily_input, Coordinate, Rectangle};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

struct Input {
    // TODO, model input here
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(9, 2024)?;

    let input = parse_data(&data);
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_data(input: &str) -> Input {
    // TODO, model input here
    Input { /* TODO, model input here */ }
}

fn solve(input: &Input) -> (usize, usize) {
    // TODO, model solve here
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "<put test input here>";

    #[test]
    fn test_part1() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).0, 14);
    }

    #[test]
    fn test_part2() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).1, 34);
    }
}
