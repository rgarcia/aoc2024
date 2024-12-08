use aochelpers::{get_daily_input, Coordinate, Rectangle};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(8, 2024)?;

    let (antennas, bounds) = parse_data(&data);
    let antennas_ref: Vec<_> = antennas.iter().collect();
    let (part1, part2) = solve(&antennas_ref, &bounds);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_data(input: &str) -> (Vec<(char, Coordinate<i32>)>, Rectangle<i32>) {
    let mut antennas = Vec::new();
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.push((
                    c,
                    Coordinate {
                        x: x as i32,
                        y: y as i32,
                    },
                ));
            }
        }
    }

    let bounds = Rectangle::new(
        Coordinate { x: 0, y: 0 },
        Coordinate {
            x: width - 1,
            y: height - 1,
        },
    );

    (antennas, bounds)
}

fn solve(antennas: &[&(char, Coordinate<i32>)], bounds: &Rectangle<i32>) -> (usize, usize) {
    let mut frequency_groups: HashMap<char, Vec<&Coordinate<i32>>> = HashMap::new();
    let mut antinodes_p1: HashSet<Coordinate<i32>> = HashSet::new();
    let mut antinodes_p2: HashSet<Coordinate<i32>> = HashSet::new();

    // Group antennas by frequency
    for (freq, pos) in antennas {
        frequency_groups.entry(*freq).or_default().push(pos);
    }

    // For each frequency group
    for positions in frequency_groups.values() {
        // Check all pairs of antennas
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let a1 = positions[i];
                let a2 = positions[j];

                let dx = a2.x - a1.x;
                let dy = a2.y - a1.y;

                // Part 1 logic
                let antinode1 = Coordinate {
                    x: a1.x - dx,
                    y: a1.y - dy,
                };
                let antinode2 = Coordinate {
                    x: a2.x + dx,
                    y: a2.y + dy,
                };

                if bounds.contains(&antinode1) {
                    antinodes_p1.insert(antinode1);
                }
                if bounds.contains(&antinode2) {
                    antinodes_p1.insert(antinode2);
                }

                // Part 2 logic
                antinodes_p2.insert(*a1);
                antinodes_p2.insert(*a2);

                let mut k = 1;
                loop {
                    let antinode1 = Coordinate {
                        x: a1.x - k * dx,
                        y: a1.y - k * dy,
                    };
                    let antinode2 = Coordinate {
                        x: a2.x + k * dx,
                        y: a2.y + k * dy,
                    };

                    let mut added = false;
                    if bounds.contains(&antinode1) {
                        antinodes_p2.insert(antinode1);
                        added = true;
                    }
                    if bounds.contains(&antinode2) {
                        antinodes_p2.insert(antinode2);
                        added = true;
                    }

                    if !added {
                        break;
                    }
                    k += 1;
                }
            }
        }
    }

    (antinodes_p1.len(), antinodes_p2.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        let (input, bounds) = parse_data(TESTDATA);
        let input_ref: Vec<_> = input.iter().collect();
        assert_eq!(solve(&input_ref, &bounds).0, 14);
    }

    #[test]
    fn test_part2() {
        let (input, bounds) = parse_data(TESTDATA);
        let input_ref: Vec<_> = input.iter().collect();
        assert_eq!(solve(&input_ref, &bounds).1, 34);
    }
}
