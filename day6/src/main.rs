use aochelpers::{get_daily_input, Coordinate, Direction};
use std::{collections::HashSet, error::Error};

struct Arena {
    walls: HashSet<Coordinate<i32>>,
    max_x: i32,
    max_y: i32,
}

impl Arena {
    fn new(walls: HashSet<Coordinate<i32>>) -> Self {
        let max_x = walls.iter().map(|c| c.x).max().unwrap();
        let max_y = walls.iter().map(|c| c.y).max().unwrap();
        Self {
            walls,
            max_x,
            max_y,
        }
    }

    fn is_in_bounds(&self, coord: &Coordinate<i32>) -> bool {
        (0..=self.max_x).contains(&coord.x) && (0..=self.max_y).contains(&coord.y)
    }

    fn contains_wall(&self, coord: &Coordinate<i32>) -> bool {
        self.walls.contains(coord)
    }

    fn add_wall(&mut self, coord: Coordinate<i32>) {
        self.walls.insert(coord);
    }

    fn clone_walls(&self) -> HashSet<Coordinate<i32>> {
        self.walls.clone()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(6, 2024)?;

    let (arena, guard) = parse_data(&data);
    let (part1, part2) = solve(&arena, guard);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn rotate_clockwise(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        _ => unimplemented!(),
    }
}

fn solve(arena: &Arena, mut guard: Coordinate<i32>) -> (usize, usize) {
    let mut current_facing = Direction::North;
    let mut visited = HashSet::new();
    let mut possible_blocks = HashSet::new();
    let starting_point = guard;

    while arena.is_in_bounds(&guard) {
        visited.insert(guard);
        while arena.contains_wall(&guard.neighbour(current_facing)) {
            current_facing = rotate_clockwise(current_facing);
        }
        let next_guard_location = guard.neighbour(current_facing);

        let mut parallel_universe = Arena::new(arena.clone_walls());
        parallel_universe.add_wall(next_guard_location);

        if !possible_blocks.contains(&next_guard_location)
            && next_guard_location != starting_point
            && is_loop(&parallel_universe, starting_point)
            && arena.is_in_bounds(&next_guard_location)
        {
            possible_blocks.insert(next_guard_location);
        }
        guard = guard.neighbour(current_facing);
    }
    (visited.len(), possible_blocks.len())
}

fn is_loop(arena: &Arena, mut guard: Coordinate<i32>) -> bool {
    let mut visited: HashSet<(Coordinate<i32>, Direction)> = HashSet::new();
    let mut current_facing: Direction = Direction::North;

    while arena.is_in_bounds(&guard) {
        if visited.contains(&(guard, current_facing)) {
            return true;
        }
        visited.insert((guard, current_facing));
        while arena.contains_wall(&guard.neighbour(current_facing)) {
            current_facing = rotate_clockwise(current_facing);
        }
        guard = guard.neighbour(current_facing);
    }
    false
}

fn parse_data(input: &str) -> (Arena, Coordinate<i32>) {
    let mut walls = HashSet::new();
    let mut guard_location = Coordinate {
        x: i32::MAX,
        y: i32::MAX,
    };
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert(Coordinate {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                '^' => {
                    guard_location = Coordinate {
                        x: x as i32,
                        y: y as i32,
                    };
                }
                _ => {}
            }
        }
    }

    (Arena::new(walls), guard_location)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        let (arena, guard) = parse_data(TESTDATA);
        assert_eq!(solve(&arena, guard).0, 41);
    }

    #[test]
    fn test_part2() {
        let (arena, guard) = parse_data(TESTDATA);
        assert_eq!(solve(&arena, guard).1, 6);
    }
}
