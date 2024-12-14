use aochelpers::{get_daily_input, Coordinate};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

#[derive(Debug)]
struct Input {
    robots: Vec<Robot>,
    width: i32,
    height: i32,
    steps: i32,
}

#[derive(Debug)]
struct Robot {
    position: Coordinate<i32>,
    velocity: Coordinate<i32>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(14, 2024)?;

    let input = parse_data(&data);
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_data(input: &str) -> Input {
    let robots = input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" v=").unwrap();
            let pos = pos.strip_prefix("p=").unwrap();
            let (px, py) = pos.split_once(',').unwrap();
            let (vx, vy) = vel.split_once(',').unwrap();

            Robot {
                position: Coordinate {
                    x: px.parse().unwrap(),
                    y: py.parse().unwrap(),
                },
                velocity: Coordinate {
                    x: vx.parse().unwrap(),
                    y: vy.parse().unwrap(),
                },
            }
        })
        .collect();

    Input {
        robots,
        width: 101,
        height: 103,
        steps: 100,
    }
}

fn solve(input: &Input) -> (usize, usize) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn solve_part1(input: &Input) -> usize {
    let mut positions: HashMap<Coordinate<i32>, usize> = HashMap::new();
    for robot in &input.robots {
        let mut pos = robot.position;
        for _ in 0..input.steps {
            pos.x = (pos.x + robot.velocity.x).rem_euclid(input.width);
            pos.y = (pos.y + robot.velocity.y).rem_euclid(input.height);
        }
        *positions.entry(pos).or_default() += 1;
    }

    let mut quadrants = vec![0; 4];
    for (pos, count) in positions {
        let middle_x = input.width / 2;
        let middle_y = input.height / 2;
        if pos.x == middle_x || pos.y == middle_y {
            continue;
        }

        let quadrant = match (pos.x < middle_x, pos.y < middle_y) {
            (true, true) => 0,
            (false, true) => 1,
            (true, false) => 2,
            (false, false) => 3,
        };
        quadrants[quadrant] += count;
    }
    quadrants.iter().product()
}

fn solve_part2(input: &Input) -> usize {
    const INV_W: i64 = 51; // Precomputed inverse of WIDTH mod HEIGHT

    let bx = find_best_offset(input, true) as i64;
    let by = find_best_offset(input, false) as i64;

    let t = bx + INV_W * (by - bx) * input.width as i64;
    t.rem_euclid(input.width as i64 * input.height as i64) as usize
}

fn calculate_variance(positions: &[(i32, i32)]) -> f64 {
    let mean = positions.iter().map(|&(x, _)| x as f64).sum::<f64>() / positions.len() as f64;
    let variance = positions
        .iter()
        .map(|&(x, _)| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / positions.len() as f64;
    variance
}

fn find_best_offset(input: &Input, use_x: bool) -> u32 {
    let modulo = if use_x { input.width } else { input.height } as u32;

    let mut best_variance = f64::MAX;
    let mut best_offset = 0;

    for offset in 0..modulo {
        let positions: Vec<_> = input
            .robots
            .iter()
            .map(|robot| {
                let new_x =
                    (robot.position.x + offset as i32 * robot.velocity.x).rem_euclid(input.width);
                let new_y =
                    (robot.position.y + offset as i32 * robot.velocity.y).rem_euclid(input.height);
                (new_x, new_y)
            })
            .collect();

        let variance = if use_x {
            calculate_variance(&positions)
        } else {
            calculate_variance(&positions.iter().map(|&(x, y)| (y, x)).collect::<Vec<_>>())
        };

        if variance < best_variance {
            best_variance = variance;
            best_offset = offset;
        }
    }
    best_offset
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        let mut input = parse_data(TESTDATA);
        input.width = 11;
        input.height = 7;
        assert_eq!(solve(&input).0, 12);
    }

    // #[test]
    // fn test_part2() {
    //     let input = parse_data(TESTDATA);
    //     assert_eq!(solve(&input).1, 0);
    // }
}
