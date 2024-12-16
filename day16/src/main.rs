use aochelpers::{get_daily_input, parse_number_grid, Coordinate, Direction, Grid};
use pathfinding::prelude::astar_bag;
use std::error::Error;

struct Input {
    grid: Grid<char>,
    start: Coordinate<usize>,
    end: Coordinate<usize>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    pos: Coordinate<usize>,
    facing: Direction,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(16, 2024)?;

    let input = parse_data(&data);
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_data(input: &str) -> Input {
    let hm = parse_number_grid(input);
    let start = hm.iter().find(|(_, &c)| c == 'S').unwrap().0.clone();
    let end = hm.iter().find(|(_, &c)| c == 'E').unwrap().0.clone();
    let mut grid = Grid::new();
    for (k, v) in hm {
        grid.insert(k, v);
    }
    Input { grid, start, end }
}

fn solve(input: &Input) -> (usize, usize) {
    let start_state = State {
        pos: input.start,
        facing: Direction::East, // Start facing east
    };

    let all_paths = astar_bag(
        &start_state,
        |state| {
            let mut successors = Vec::new();

            // Try moving forward
            let next_pos = state.pos.neighbour(state.facing);
            if input.grid.get(&next_pos) == Some('.') || input.grid.get(&next_pos) == Some('E') {
                successors.push((
                    State {
                        pos: next_pos,
                        facing: state.facing,
                    },
                    1,
                ));
            }

            // Try turning left
            let left = match state.facing {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
                _ => unreachable!(),
            };
            successors.push((
                State {
                    pos: state.pos,
                    facing: left,
                },
                1000,
            ));

            let right = match state.facing {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                _ => unreachable!(),
            };
            successors.push((
                State {
                    pos: state.pos,
                    facing: right,
                },
                1000,
            ));

            successors
        },
        |state| (state.pos.x.abs_diff(input.end.x) + state.pos.y.abs_diff(input.end.y)) as u32,
        |state| state.pos == input.end,
    );

    if let Some((paths, cost)) = all_paths {
        // Collect all unique positions from optimal paths
        let mut visited = std::collections::HashSet::new();
        paths.for_each(|path| {
            for state in path {
                visited.insert(state.pos);
            }
        });

        (cost as usize, visited.len())
    } else {
        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    const TESTDATA2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part1() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).0, 7036);
        let input = parse_data(TESTDATA2);
        assert_eq!(solve(&input).0, 11048);
    }

    #[test]
    fn test_part2() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).1, 45);
        let input = parse_data(TESTDATA2);
        assert_eq!(solve(&input).1, 64);
    }
}
