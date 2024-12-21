use aochelpers::{get_daily_input, Coordinate};
use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
};

struct Input {
    codes: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(21, 2024)?;
    let input = parse_data(&data);
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

fn parse_data(input: &str) -> Input {
    Input {
        codes: input.lines().map(|l| l.to_string()).collect(),
    }
}

const PAD1: &[&[u8]] = &[b"789", b"456", b"123", b" 0A"];
const PAD2: &[&[u8]] = &[b" ^A", b"<v>"];

fn pad_move(pos: Coordinate<i64>, m: u8, pad: &[&[u8]]) -> (Coordinate<i64>, Option<u8>) {
    let next_pos = match m {
        b'<' => pos + Coordinate { x: 0, y: -1 },
        b'^' => pos + Coordinate { x: -1, y: 0 },
        b'>' => pos + Coordinate { x: 0, y: 1 },
        b'v' => pos + Coordinate { x: 1, y: 0 },
        b'A' => pos,
        _ => unreachable!(),
    };
    (
        next_pos,
        if m == b'A' {
            Some(pad[pos.x as usize][pos.y as usize])
        } else {
            None
        },
    )
}

fn calculate_cost(
    cache: &mut HashMap<(u8, u8, usize), usize>,
    goal: u8,
    prev_m: u8,
    pads: usize,
) -> usize {
    if pads == 0 {
        return 1;
    }
    if let Some(&d) = cache.get(&(goal, prev_m, pads)) {
        return d;
    }
    let start = match prev_m {
        b'^' => Coordinate { x: 0, y: 1 },
        b'A' => Coordinate { x: 0, y: 2 },
        b'<' => Coordinate { x: 1, y: 0 },
        b'v' => Coordinate { x: 1, y: 1 },
        b'>' => Coordinate { x: 1, y: 2 },
        _ => unreachable!(),
    };
    let mut q = BinaryHeap::from([(0, start, b'A', 0)]);
    while let Some((d, pos, prev, out)) = q.pop() {
        let d = (-d) as usize;
        if out == goal {
            cache.insert((goal, prev_m, pads), d);
            return d;
        }
        for &m in b"A^<v>" {
            let (pos, x) = pad_move(pos, m, PAD2);
            if *PAD2
                .get(pos.x as usize)
                .and_then(|row| row.get(pos.y as usize))
                .unwrap_or(&b' ')
                == b' '
            {
                continue;
            }
            let x = x.unwrap_or(0);
            if x != 0 && x != goal {
                continue;
            }
            let d = d + calculate_cost(cache, m, prev, pads - 1);
            q.push((-(d as i64), pos, m, x));
        }
    }
    unreachable!()
}

fn solve_inner(cache: &mut HashMap<(u8, u8, usize), usize>, code: &[u8], pads: usize) -> usize {
    let mut q: BinaryHeap<(i64, Coordinate<i64>, u8, usize)> =
        BinaryHeap::from([(0, Coordinate { x: 3, y: 2 }, b'A', 0)]);
    let mut seen = HashMap::new();
    while let Some((d, pos, prev, l)) = q.pop() {
        let d = (-d) as usize;
        if l == code.len() {
            return d;
        }
        let k = (pos, prev, l);
        if seen.contains_key(&k) {
            continue;
        }
        seen.insert(k, d);
        for &m in b"A^<v>" {
            let (next_pos, x) = pad_move(pos, m, PAD1);
            if next_pos.x < 0
                || next_pos.y < 0
                || next_pos.x >= PAD1.len() as i64
                || next_pos.y >= PAD1[0].len() as i64
                || PAD1[next_pos.x as usize][next_pos.y as usize] == b' '
            {
                continue;
            }
            let mut l = l;
            if let Some(x) = x {
                if x != code[l] {
                    continue;
                }
                l += 1;
            }
            let d = d + calculate_cost(cache, m, prev, pads);
            q.push((-(d as i64), next_pos, m, l));
        }
    }
    unreachable!()
}

fn solve(input: &Input) -> (usize, usize) {
    let mut cache = HashMap::new();
    let mut p1 = 0;
    let mut p2 = 0;
    for code in &input.codes {
        let n = code.strip_suffix('A').unwrap().parse::<usize>().unwrap();
        p1 += n * solve_inner(&mut cache, code.as_bytes(), 2);
        p2 += n * solve_inner(&mut cache, code.as_bytes(), 25);
    }
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_part1() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).0, 126384);
    }

    #[test]
    fn test_part2() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).1, 154115708116294);
    }
}
