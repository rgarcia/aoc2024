use aochelpers::{get_daily_input, ScoredItem};
use std::{
    collections::{BinaryHeap, HashSet},
    error::Error,
};

#[derive(Debug, Clone)]
struct Equation {
    target: i64,
    numbers: Vec<i64>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct PartialSolution {
    current_value: i64,
    ops_used: usize,
    ops: Vec<char>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(7, 2024)?;

    let equations = parse_data(&data);
    let (part1, part2) = solve(&equations);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn solve(equations: &[Equation]) -> (i64, i64) {
    let part1: i64 = equations
        .iter()
        .filter(|eq| solve_equation(eq, false))
        .map(|eq| eq.target)
        .sum();

    let part2: i64 = equations
        .iter()
        .filter(|eq| solve_equation(eq, true))
        .map(|eq| eq.target)
        .sum();

    (part1, part2)
}

fn parse_data(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (target, nums) = line.split_once(": ").unwrap();
            Equation {
                target: target.parse().unwrap(),
                numbers: nums
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn solve_equation(eq: &Equation, allow_concat: bool) -> bool {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    // Start with first number
    let initial = PartialSolution {
        current_value: eq.numbers[0],
        ops_used: 0,
        ops: vec![],
    };

    heap.push(ScoredItem {
        cost: (eq.target - initial.current_value).abs(),
        item: initial,
    });

    while let Some(ScoredItem { item: partial, .. }) = heap.pop() {
        if !seen.insert((partial.current_value, partial.ops_used)) {
            continue;
        }

        if partial.ops_used == eq.numbers.len() - 1 {
            if partial.current_value == eq.target {
                return true;
            }
            continue;
        }

        let next_num = eq.numbers[partial.ops_used + 1];

        // Try addition
        let mut new_ops = partial.ops.clone();
        new_ops.push('+');
        let add_solution = PartialSolution {
            current_value: partial.current_value + next_num,
            ops_used: partial.ops_used + 1,
            ops: new_ops,
        };
        heap.push(ScoredItem {
            cost: (eq.target - add_solution.current_value).abs(),
            item: add_solution,
        });

        // Try multiplication
        let mut new_ops = partial.ops.clone();
        new_ops.push('*');
        let mul_solution = PartialSolution {
            current_value: partial.current_value * next_num,
            ops_used: partial.ops_used + 1,
            ops: new_ops,
        };
        heap.push(ScoredItem {
            cost: (eq.target - mul_solution.current_value).abs(),
            item: mul_solution,
        });

        // Try concatenation if allowed
        if allow_concat {
            let mut new_ops = partial.ops.clone();
            new_ops.push('|');
            // Convert current value to string, append next number, parse back to i64
            let concat_value = format!("{}{}", partial.current_value, next_num)
                .parse()
                .unwrap();
            let concat_solution = PartialSolution {
                current_value: concat_value,
                ops_used: partial.ops_used + 1,
                ops: new_ops,
            };
            heap.push(ScoredItem {
                cost: (eq.target - concat_solution.current_value).abs(),
                item: concat_solution,
            });
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        let equations = parse_data(TESTDATA);
        assert_eq!(solve(&equations).0, 3749);
    }

    #[test]
    fn test_part2() {
        let equations = parse_data(TESTDATA);
        assert_eq!(solve(&equations).1, 11387);
    }
}
