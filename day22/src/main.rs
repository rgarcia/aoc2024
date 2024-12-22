use aochelpers::get_daily_input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(22, 2024)?;
    let (part1, part2) = solve(&data);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn solve(input: &str) -> (i64, i64) {
    let (mut p1, mut p2) = (0, HashMap::new());
    let mut seen = HashSet::new();
    for l in input.lines() {
        let mut ps = [0; 2000];
        let mut p = l.parse::<i64>().unwrap();
        for i in 0..2000 {
            p = (p ^ (p * 64)) % 16777216;
            p = (p ^ (p / 32)) % 16777216;
            p = (p ^ (p * 2048)) % 16777216;
            ps[i] = p % 10;
        }
        p1 += p;

        seen.clear();
        for (a, b, c, d, e) in ps.iter().tuple_windows() {
            let k = (b - a) + (c - b) * 20 + (d - c) * 400 + (e - d) * 8000;
            if seen.insert(k) {
                *p2.entry(k).or_default() += *e;
            }
        }
    }
    (p1, *p2.values().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "1
10
100
2024";

    const TESTDATA2: &str = "1
2
3
2024";

    #[test]
    fn test_part1() {
        assert_eq!(solve(TESTDATA).0, 37327623);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(TESTDATA2).1, 23);
    }
}
