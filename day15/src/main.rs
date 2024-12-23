use aochelpers::{get_daily_input, Coordinate, Direction, Grid};
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Cell {
    Robot,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Empty,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(15, 2024)?;
    let (grid, directions) = parse_data(&data);
    println!("Part 1: {}", part1(grid, &directions));
    let (grid, directions) = parse_data_wide(&data);
    println!("Part 2: {}", part2(grid, &directions));
    Ok(())
}

fn part1(mut grid: Grid<Cell>, directions: &Vec<Direction>) -> usize {
    let mut robot: Coordinate<usize> = grid
        .iter()
        .find(|(_, o)| o == &Cell::Robot)
        .map(|(c, _)| c)
        .expect("There is no robot here");
    grid.insert(robot, Cell::Empty);

    for direction in directions {
        let mut neighbour: Coordinate<usize> = robot.neighbour(*direction);
        match grid.get(&neighbour) {
            Some(Cell::Wall) | Some(Cell::Robot) => {}
            Some(Cell::Empty) => {
                grid.insert(neighbour, Cell::Empty);
                robot = neighbour;
            }
            Some(Cell::Box) => {
                while grid.get(&neighbour) == Some(Cell::Box) {
                    neighbour = neighbour.neighbour(*direction);
                }
                match grid.get(&neighbour) {
                    Some(Cell::Wall) | Some(Cell::Robot) => {}
                    Some(Cell::Empty) => {
                        grid.insert(neighbour, Cell::Box);
                        grid.insert(robot.neighbour(*direction), Cell::Empty);
                        robot = robot.neighbour(*direction);
                    }
                    None | Some(Cell::Box) | Some(Cell::BoxLeft) | Some(Cell::BoxRight) => {
                        unimplemented!()
                    }
                }
            }
            None | Some(Cell::BoxLeft) | Some(Cell::BoxRight) => unimplemented!(),
        }
    }
    grid.iter::<usize>()
        .filter(|(_, s)| s == &Cell::Box)
        .map(|(c, _)| c.x + c.y * 100)
        .sum()
}

fn part2(mut grid: Grid<Cell>, directions: &Vec<Direction>) -> usize {
    let mut robot: Coordinate<usize> = grid
        .iter()
        .find(|(_, o)| o == &Cell::Robot)
        .map(|(c, _)| c)
        .expect("There is no robot here");
    grid.insert(robot, Cell::Empty);

    for direction in directions {
        let neighbour = robot.neighbour(*direction);
        match grid.get(&neighbour) {
            Some(Cell::Wall) | Some(Cell::Robot) => {}
            Some(Cell::Empty) => {
                grid.insert(neighbour, Cell::Empty);
                robot = neighbour;
            }
            Some(Cell::BoxLeft) | Some(Cell::BoxRight) => {
                if can_move(neighbour, *direction, &grid) {
                    move_box(neighbour, *direction, &mut grid);
                    grid.insert(neighbour, Cell::Empty);
                    robot = neighbour;
                }
            }
            _ => unimplemented!(),
        }
    }
    grid.iter::<usize>()
        .filter(|(_, s)| s == &Cell::BoxLeft)
        .map(|(c, _)| c.x + c.y * 100)
        .sum()
}

fn can_move(location: Coordinate<usize>, direction: Direction, grid: &Grid<Cell>) -> bool {
    match direction {
        Direction::North => match grid.get(&location) {
            Some(Cell::BoxLeft) => {
                match (
                    grid.get(&Coordinate {
                        x: location.x,
                        y: location.y - 1,
                    }),
                    grid.get(&Coordinate {
                        x: location.x + 1,
                        y: location.y - 1,
                    }),
                ) {
                    (Some(Cell::Empty), Some(Cell::Empty)) => true,
                    (Some(Cell::Wall), _) | (_, Some(Cell::Wall)) => false,
                    (Some(Cell::BoxLeft), Some(Cell::BoxRight)) => can_move(
                        Coordinate {
                            x: location.x,
                            y: location.y - 1,
                        },
                        direction,
                        grid,
                    ),
                    (Some(Cell::BoxRight), Some(Cell::Empty)) => can_move(
                        Coordinate {
                            x: location.x - 1,
                            y: location.y - 1,
                        },
                        direction,
                        grid,
                    ),
                    (Some(Cell::Empty), Some(Cell::BoxLeft)) => can_move(
                        Coordinate {
                            x: location.x + 1,
                            y: location.y - 1,
                        },
                        direction,
                        grid,
                    ),
                    (Some(Cell::BoxRight), Some(Cell::BoxLeft)) => {
                        can_move(
                            Coordinate {
                                x: location.x - 1,
                                y: location.y - 1,
                            },
                            direction,
                            grid,
                        ) && can_move(
                            Coordinate {
                                x: location.x + 1,
                                y: location.y - 1,
                            },
                            direction,
                            grid,
                        )
                    }
                    (_, _) => unimplemented!(),
                }
            }
            Some(Cell::BoxRight) => can_move(
                Coordinate {
                    x: location.x - 1,
                    y: location.y,
                },
                direction,
                grid,
            ),
            _ => unimplemented!(),
        },
        Direction::East => {
            let target_location = Coordinate {
                x: location.x + 2,
                y: location.y,
            };
            match grid.get(&target_location) {
                Some(Cell::Empty) => true,
                Some(Cell::Wall) => false,
                Some(Cell::BoxLeft) => can_move(target_location, direction, grid),
                None | Some(Cell::Box) | Some(Cell::BoxRight) | Some(Cell::Robot) => {
                    unimplemented!()
                }
            }
        }
        Direction::South => match grid.get(&location) {
            Some(Cell::BoxLeft) => {
                match (
                    grid.get(&Coordinate {
                        x: location.x,
                        y: location.y + 1,
                    }),
                    grid.get(&Coordinate {
                        x: location.x + 1,
                        y: location.y + 1,
                    }),
                ) {
                    (Some(Cell::Empty), Some(Cell::Empty)) => true,
                    (Some(Cell::Wall), _) | (_, Some(Cell::Wall)) => false,
                    (Some(Cell::BoxLeft), Some(Cell::BoxRight)) => can_move(
                        Coordinate {
                            x: location.x,
                            y: location.y + 1,
                        },
                        direction,
                        grid,
                    ),
                    (Some(Cell::BoxRight), Some(Cell::Empty)) => can_move(
                        Coordinate {
                            x: location.x - 1,
                            y: location.y + 1,
                        },
                        direction,
                        grid,
                    ),
                    (Some(Cell::Empty), Some(Cell::BoxLeft)) => can_move(
                        Coordinate {
                            x: location.x + 1,
                            y: location.y + 1,
                        },
                        direction,
                        grid,
                    ),
                    (Some(Cell::BoxRight), Some(Cell::BoxLeft)) => {
                        can_move(
                            Coordinate {
                                x: location.x - 1,
                                y: location.y + 1,
                            },
                            direction,
                            grid,
                        ) && can_move(
                            Coordinate {
                                x: location.x + 1,
                                y: location.y + 1,
                            },
                            direction,
                            grid,
                        )
                    }
                    (_, _) => unimplemented!(),
                }
            }
            Some(Cell::BoxRight) => can_move(
                Coordinate {
                    x: location.x - 1,
                    y: location.y,
                },
                direction,
                grid,
            ),
            _ => unimplemented!(),
        },
        Direction::West => {
            let target_location: Coordinate<usize> = Coordinate {
                x: location.x - 2,
                y: location.y,
            };
            match grid.get(&target_location) {
                Some(Cell::Empty) => true,
                Some(Cell::Wall) => false,
                Some(Cell::BoxRight) => can_move(target_location, direction, grid),
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

fn move_box(location: Coordinate<usize>, direction: Direction, grid: &mut Grid<Cell>) {
    match grid.get(&location) {
        Some(Cell::BoxLeft) => match direction {
            Direction::North => {
                match (
                    grid.get(&Coordinate {
                        x: location.x,
                        y: location.y - 1,
                    }),
                    grid.get(&Coordinate {
                        x: location.x + 1,
                        y: location.y - 1,
                    }),
                ) {
                    (Some(Cell::Empty), Some(Cell::Empty)) => {}
                    (Some(Cell::BoxLeft), Some(Cell::BoxRight)) => {
                        move_box(
                            Coordinate {
                                x: location.x,
                                y: location.y - 1,
                            },
                            direction,
                            grid,
                        );
                    }
                    (Some(Cell::BoxRight), Some(Cell::Empty)) => {
                        move_box(
                            Coordinate {
                                x: location.x - 1,
                                y: location.y - 1,
                            },
                            direction,
                            grid,
                        );
                    }
                    (Some(Cell::Empty), Some(Cell::BoxLeft)) => {
                        move_box(
                            Coordinate {
                                x: location.x + 1,
                                y: location.y - 1,
                            },
                            direction,
                            grid,
                        );
                    }
                    (Some(Cell::BoxRight), Some(Cell::BoxLeft)) => {
                        move_box(
                            Coordinate {
                                x: location.x + 1,
                                y: location.y - 1,
                            },
                            direction,
                            grid,
                        );
                        move_box(
                            Coordinate {
                                x: location.x - 1,
                                y: location.y - 1,
                            },
                            direction,
                            grid,
                        );
                    }
                    (_, _) => unimplemented!(),
                };
                grid.insert(
                    Coordinate {
                        x: location.x,
                        y: location.y - 1,
                    },
                    Cell::BoxLeft,
                );
                grid.insert(
                    Coordinate {
                        x: location.x + 1,
                        y: location.y - 1,
                    },
                    Cell::BoxRight,
                );
                grid.insert(location, Cell::Empty);
                grid.insert(
                    Coordinate {
                        x: location.x + 1,
                        y: location.y,
                    },
                    Cell::Empty,
                );
            }
            Direction::South => {
                match (
                    grid.get(&Coordinate {
                        x: location.x,
                        y: location.y + 1,
                    }),
                    grid.get(&Coordinate {
                        x: location.x + 1,
                        y: location.y + 1,
                    }),
                ) {
                    (Some(Cell::Empty), Some(Cell::Empty)) => {}
                    (Some(Cell::BoxLeft), Some(Cell::BoxRight)) => {
                        move_box(
                            Coordinate {
                                x: location.x,
                                y: location.y + 1,
                            },
                            direction,
                            grid,
                        );
                    }
                    (Some(Cell::BoxRight), Some(Cell::Empty)) => {
                        move_box(
                            Coordinate {
                                x: location.x - 1,
                                y: location.y + 1,
                            },
                            direction,
                            grid,
                        );
                    }
                    (Some(Cell::Empty), Some(Cell::BoxLeft)) => {
                        move_box(
                            Coordinate {
                                x: location.x + 1,
                                y: location.y + 1,
                            },
                            direction,
                            grid,
                        );
                    }
                    (Some(Cell::BoxRight), Some(Cell::BoxLeft)) => {
                        move_box(
                            Coordinate {
                                x: location.x + 1,
                                y: location.y + 1,
                            },
                            direction,
                            grid,
                        );
                        move_box(
                            Coordinate {
                                x: location.x - 1,
                                y: location.y + 1,
                            },
                            direction,
                            grid,
                        );
                    }
                    (_, _) => unimplemented!(),
                };
                grid.insert(
                    Coordinate {
                        x: location.x,
                        y: location.y + 1,
                    },
                    Cell::BoxLeft,
                );
                grid.insert(
                    Coordinate {
                        x: location.x + 1,
                        y: location.y + 1,
                    },
                    Cell::BoxRight,
                );
                grid.insert(location, Cell::Empty);
                grid.insert(
                    Coordinate {
                        x: location.x + 1,
                        y: location.y,
                    },
                    Cell::Empty,
                );
            }
            Direction::East => {
                match grid.get(&Coordinate {
                    x: location.x + 2,
                    y: location.y,
                }) {
                    Some(Cell::Empty) => {}
                    Some(Cell::BoxLeft) => {
                        move_box(
                            Coordinate {
                                x: location.x + 2,
                                y: location.y,
                            },
                            direction,
                            grid,
                        );
                    }
                    _ => unimplemented!(),
                };
                grid.insert(
                    Coordinate {
                        x: location.x + 2,
                        y: location.y,
                    },
                    Cell::BoxRight,
                );
                grid.insert(
                    Coordinate {
                        x: location.x + 1,
                        y: location.y,
                    },
                    Cell::BoxLeft,
                );
                grid.insert(location, Cell::Empty);
            }
            Direction::West => {
                match grid.get(&Coordinate {
                    x: location.x - 1,
                    y: location.y,
                }) {
                    Some(Cell::Empty) => {}
                    Some(Cell::BoxRight) => {
                        move_box(
                            Coordinate {
                                x: location.x - 1,
                                y: location.y,
                            },
                            direction,
                            grid,
                        );
                    }
                    _ => unimplemented!(),
                };
                grid.insert(
                    Coordinate {
                        x: location.x - 1,
                        y: location.y,
                    },
                    Cell::BoxLeft,
                );
                grid.insert(location, Cell::BoxRight);
                grid.insert(
                    Coordinate {
                        x: location.x + 1,
                        y: location.y,
                    },
                    Cell::Empty,
                );
            }
            _ => unimplemented!(),
        },
        Some(Cell::BoxRight) => move_box(
            Coordinate {
                x: location.x - 1,
                y: location.y,
            },
            direction,
            grid,
        ),
        _ => unimplemented!(),
    }
}

fn parse_data(data: &str) -> (Grid<Cell>, Vec<Direction>) {
    let mut sections = data.split("\n\n");
    let mut grid = Grid::new();
    for (y, line) in sections.next().unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(
                Coordinate { x, y },
                match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'O' => Cell::Box,
                    '@' => Cell::Robot,
                    _ => unimplemented!(),
                },
            );
        }
    }
    let directions = sections
        .next()
        .unwrap()
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '^' => Direction::North,
            '<' => Direction::West,
            '>' => Direction::East,
            'v' => Direction::South,
            _ => {
                println!("Can't understand direction '{}'", c);
                unimplemented!();
            }
        })
        .collect();
    (grid, directions)
}

fn parse_data_wide(data: &str) -> (Grid<Cell>, Vec<Direction>) {
    let mut sections = data.split("\n\n");
    let mut grid = Grid::new();
    for (y, line) in sections.next().unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(
                Coordinate { x: x * 2, y },
                match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'O' => Cell::BoxLeft,
                    '@' => Cell::Robot,
                    _ => unimplemented!(),
                },
            );
            grid.insert(
                Coordinate { x: x * 2 + 1, y },
                match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'O' => Cell::BoxRight,
                    '@' => Cell::Empty,
                    _ => unimplemented!(),
                },
            );
        }
    }
    let directions = sections
        .next()
        .unwrap()
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '^' => Direction::North,
            '<' => Direction::West,
            '>' => Direction::East,
            'v' => Direction::South,
            _ => {
                println!("Can't understand direction '{}'", c);
                unimplemented!();
            }
        })
        .collect();
    (grid, directions)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SMALLTEST: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const BIGTEST: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part1() {
        let (grid, directions) = parse_data(SMALLTEST);
        assert_eq!(part1(grid, &directions), 2028);
    }

    #[test]
    fn test_part2() {
        let (grid, directions) = parse_data_wide(BIGTEST);
        assert_eq!(part2(grid, &directions), 9021);
    }

    #[test]
    fn test_part1_big() {
        let (grid, directions) = parse_data(BIGTEST);
        assert_eq!(part1(grid, &directions), 10092);
    }
}
