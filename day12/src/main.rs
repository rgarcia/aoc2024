use aochelpers::{get_daily_input, parse_number_grid, Coordinate, Rectangle};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

#[derive(Debug)]
struct Region {
    coordinates: HashSet<Coordinate<i32>>,
}

impl Region {
    fn new(coord: Coordinate<i32>) -> Self {
        let mut coordinates = HashSet::new();
        coordinates.insert(coord);
        Region { coordinates }
    }

    fn area(&self) -> usize {
        self.coordinates.len()
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = 0;
        for coord in &self.coordinates {
            // Check all 4 directions
            for neighbor in [
                Coordinate {
                    x: coord.x + 1,
                    y: coord.y,
                },
                Coordinate {
                    x: coord.x - 1,
                    y: coord.y,
                },
                Coordinate {
                    x: coord.x,
                    y: coord.y + 1,
                },
                Coordinate {
                    x: coord.x,
                    y: coord.y - 1,
                },
            ] {
                if !self.coordinates.contains(&neighbor) {
                    perimeter += 1;
                }
            }
        }
        perimeter
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn count_sides(&self) -> usize {
        let mut corners = 0;

        for coord in &self.coordinates {
            // Check adjacent cells in clockwise order
            let adjacent = [
                Coordinate {
                    x: coord.x - 1,
                    y: coord.y,
                }, // left
                Coordinate {
                    x: coord.x,
                    y: coord.y - 1,
                }, // top
                Coordinate {
                    x: coord.x + 1,
                    y: coord.y,
                }, // right
                Coordinate {
                    x: coord.x,
                    y: coord.y + 1,
                }, // bottom
            ];

            // Check diagonal cells in clockwise order
            let diagonal = [
                Coordinate {
                    x: coord.x - 1,
                    y: coord.y - 1,
                }, // top-left
                Coordinate {
                    x: coord.x + 1,
                    y: coord.y - 1,
                }, // top-right
                Coordinate {
                    x: coord.x + 1,
                    y: coord.y + 1,
                }, // bottom-right
                Coordinate {
                    x: coord.x - 1,
                    y: coord.y + 1,
                }, // bottom-left
            ];

            // Check each corner
            for i in 0..4 {
                let adj1 = self.coordinates.contains(&adjacent[i]);
                let adj2 = self.coordinates.contains(&adjacent[(i + 1) % 4]);

                if !adj1 && !adj2 {
                    // Convex corner (outside corner)
                    corners += 1;
                } else if adj1 && adj2 && !self.coordinates.contains(&diagonal[i]) {
                    // Concave corner (inside corner)
                    corners += 1;
                }
            }
        }

        corners
    }

    fn price_part2(&self) -> usize {
        self.area() * self.count_sides()
    }
}

struct Input {
    grid: HashMap<Coordinate<i32>, char>,
    bounds: Rectangle<i32>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(12, 2024)?;

    let input = parse_data(&data);
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_data(input: &str) -> Input {
    let grid = parse_number_grid(input);
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len() as i32;
    let width = lines.first().map_or(0, |line| line.len()) as i32;
    let bounds = Rectangle {
        top_left: Coordinate { x: 0, y: 0 },
        bottom_right: Coordinate {
            x: width - 1,
            y: height - 1,
        },
    };
    Input { grid, bounds }
}

fn solve(input: &Input) -> (usize, usize) {
    let mut regions: Vec<Region> = Vec::new();
    let mut visited: HashSet<Coordinate<i32>> = HashSet::new();

    // Iterate through all coordinates
    for y in input.bounds.top_left.y..=input.bounds.bottom_right.y {
        for x in input.bounds.top_left.x..=input.bounds.bottom_right.x {
            let coord = Coordinate { x, y };
            if visited.contains(&coord) {
                continue;
            }

            if let Some(&plant_type) = input.grid.get(&coord) {
                // Create new region and expand it using BFS
                let mut region = Region::new(coord);
                let mut queue = vec![coord];
                visited.insert(coord);

                while let Some(current) = queue.pop() {
                    // Check all 4 directions
                    for neighbor in [
                        Coordinate {
                            x: current.x + 1,
                            y: current.y,
                        },
                        Coordinate {
                            x: current.x - 1,
                            y: current.y,
                        },
                        Coordinate {
                            x: current.x,
                            y: current.y + 1,
                        },
                        Coordinate {
                            x: current.x,
                            y: current.y - 1,
                        },
                    ] {
                        if !visited.contains(&neighbor) {
                            if let Some(&neighbor_type) = input.grid.get(&neighbor) {
                                if neighbor_type == plant_type {
                                    queue.push(neighbor);
                                    region.coordinates.insert(neighbor);
                                    visited.insert(neighbor);
                                }
                            }
                        }
                    }
                }

                regions.push(region);
            }
        }
    }

    let total_price: usize = regions.iter().map(|r| r.price()).sum();
    let total_price_part2: usize = regions.iter().map(|r| r.price_part2()).sum();

    (total_price, total_price_part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).0, 1930);
    }

    #[test]
    fn test_part2() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).1, 1206);
    }
}
