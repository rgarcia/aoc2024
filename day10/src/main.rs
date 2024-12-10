use aochelpers::{get_daily_input, parse_number_grid, Coordinate};
use pathfinding::prelude::dijkstra;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

struct Input {
    grid: HashMap<Coordinate<i32>, i32>,
    trailheads: HashSet<Coordinate<i32>>,
    trail_ends: HashSet<Coordinate<i32>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(10, 2024)?;

    let input = parse_data(&data);
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_data(input: &str) -> Input {
    let grid = parse_number_grid(input);
    let trailheads = grid
        .iter()
        .filter(|(_, &v)| v == 0)
        .map(|(k, _)| k.clone())
        .collect();
    let trail_ends = grid
        .iter()
        .filter(|(_, &v)| v == 9)
        .map(|(k, _)| k.clone())
        .collect();

    Input {
        grid,
        trailheads,
        trail_ends,
    }
}

fn solve(input: &Input) -> (usize, usize) {
    let total_score: usize = input
        .trailheads
        .iter()
        .map(|start| score_trailhead(start, input))
        .sum();

    let total_rating: usize = input
        .trailheads
        .iter()
        .map(|start| rate_trailhead(start, input))
        .sum();

    (total_score, total_rating)
}

fn score_trailhead(start: &Coordinate<i32>, input: &Input) -> usize {
    input
        .trail_ends
        .iter()
        .filter(|end| can_reach_end(start, end, input))
        .count()
}

fn can_reach_end(start: &Coordinate<i32>, end: &Coordinate<i32>, input: &Input) -> bool {
    let successors = |pos: &Coordinate<i32>| {
        let current_height = *input.grid.get(pos).unwrap();
        pos.neighbours()
            .into_iter()
            .filter(|next| {
                if let Some(&next_height) = input.grid.get(next) {
                    next_height == current_height + 1
                } else {
                    false
                }
            })
            .map(|next| (next, 1))
            .collect::<Vec<_>>()
    };

    dijkstra(start, successors, |pos| pos == end).is_some()
}

fn rate_trailhead(start: &Coordinate<i32>, input: &Input) -> usize {
    let mut stack = vec![(start.clone(), vec![start.clone()])];
    let mut distinct_trails = 0;

    while let Some((current, path)) = stack.pop() {
        if input.trail_ends.contains(&current) {
            distinct_trails += 1;
            continue;
        }

        for (next, _) in successors(&current, input) {
            if !path.contains(&next) {
                let mut new_path = path.clone();
                new_path.push(next.clone());
                stack.push((next, new_path));
            }
        }
    }

    distinct_trails
}

fn successors(pos: &Coordinate<i32>, input: &Input) -> Vec<(Coordinate<i32>, usize)> {
    let current_height = *input.grid.get(pos).unwrap();
    pos.neighbours()
        .into_iter()
        .filter(|next| {
            if let Some(&next_height) = input.grid.get(next) {
                next_height == current_height + 1
            } else {
                false
            }
        })
        .map(|next| (next, 1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).0, 36);
    }

    #[test]
    fn test_part2() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).1, 81);
    }
}
