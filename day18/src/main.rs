use aochelpers::{get_daily_input, Coordinate, Grid};
use pathfinding::prelude::astar;
use std::error::Error;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Corrupted,
}

struct Input {
    bytes: Vec<Coordinate<usize>>,
    grid_size: usize,
    steps: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(18, 2024)?;

    let input = parse_data(&data, 71, 1024);
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_data(input: &str, grid_size: usize, steps: usize) -> Input {
    Input {
        bytes: input
            .lines()
            .map(|line| {
                let mut parts = line.split(',');
                let x = parts.next().unwrap().parse::<usize>().unwrap();
                let y = parts.next().unwrap().parse::<usize>().unwrap();
                Coordinate { x, y }
            })
            .collect(),
        grid_size,
        steps,
    }
}

fn solve(input: &Input) -> (usize, String) {
    let part1 = find_path(&input.bytes, input.steps, input.grid_size);
    let part2 = find_blocking_byte(&input.bytes, input.grid_size);
    (part1, format!("{},{}", part2.x, part2.y))
}

fn get_neighbors(
    pos: &Coordinate<usize>,
    grid: &Grid<Cell>,
    grid_size: usize,
) -> Vec<(Coordinate<usize>, u32)> {
    let mut neighbors = Vec::with_capacity(4);

    // Add neighbors only if we won't underflow/overflow
    if pos.y > 0 {
        neighbors.push(Coordinate {
            x: pos.x,
            y: pos.y - 1,
        });
    } // North
    if pos.y + 1 < grid_size {
        neighbors.push(Coordinate {
            x: pos.x,
            y: pos.y + 1,
        });
    } // South
    if pos.x > 0 {
        neighbors.push(Coordinate {
            x: pos.x - 1,
            y: pos.y,
        });
    } // West
    if pos.x + 1 < grid_size {
        neighbors.push(Coordinate {
            x: pos.x + 1,
            y: pos.y,
        });
    } // East

    neighbors
        .into_iter()
        .filter(|p| grid.get(p) == Some(Cell::Empty))
        .map(|p| (p, 1))
        .collect()
}

fn find_path_with_grid(grid: &Grid<Cell>, grid_size: usize) -> Option<Vec<Coordinate<usize>>> {
    let start = Coordinate { x: 0, y: 0 };
    let goal = Coordinate {
        x: grid_size - 1,
        y: grid_size - 1,
    };

    astar(
        &start,
        |pos| get_neighbors(pos, grid, grid_size),
        |pos| pos.manhattan_distance(&goal) as u32,
        |pos| *pos == goal,
    )
    .map(|(path, _)| path)
}

fn find_path(bytes: &[Coordinate<usize>], steps: usize, grid_size: usize) -> usize {
    let mut grid: Grid<Cell> = Grid::new();

    // Initialize empty grid first
    for y in 0..grid_size {
        for x in 0..grid_size {
            grid.insert(Coordinate { x, y }, Cell::Empty);
        }
    }

    // Fill corrupted bytes up to max_bytes
    for byte in bytes.iter().take(steps) {
        grid.insert(*byte, Cell::Corrupted);
    }

    find_path_with_grid(&grid, grid_size).unwrap().len() - 1
}

fn find_blocking_byte(bytes: &[Coordinate<usize>], grid_size: usize) -> Coordinate<usize> {
    let mut low = 0;
    let mut high = bytes.len();

    while low < high {
        let mid = (low + high) / 2;
        if path_exists(&bytes[..mid], grid_size) {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    bytes[low - 1]
}

fn path_exists(bytes: &[Coordinate<usize>], grid_size: usize) -> bool {
    let mut grid: Grid<Cell> = Grid::new();

    // Initialize empty grid first
    for y in 0..grid_size {
        for x in 0..grid_size {
            grid.insert(Coordinate { x, y }, Cell::Empty);
        }
    }

    // Fill corrupted bytes
    for byte in bytes.iter() {
        grid.insert(*byte, Cell::Corrupted);
    }

    find_path_with_grid(&grid, grid_size).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test() {
        let input = parse_data(TESTDATA, 7, 12);
        assert_eq!(solve(&input).0, 22, "6,1");
    }
}
