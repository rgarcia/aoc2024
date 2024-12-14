use aochelpers::get_daily_input;
use std::error::Error;

struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

struct Input {
    machines: Vec<Machine>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(13, 2024)?;

    let input = parse_data(&data);
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_data(input: &str) -> Input {
    let machines = input
        .split("\n\n")
        .map(|block| {
            let lines: Vec<&str> = block.lines().collect();
            let (a_x, a_y) = parse_button(lines[0]);
            let (b_x, b_y) = parse_button(lines[1]);
            let (prize_x, prize_y) = parse_prize(lines[2]);
            Machine {
                a_x,
                a_y,
                b_x,
                b_y,
                prize_x,
                prize_y,
            }
        })
        .collect();
    Input { machines }
}

fn parse_button(line: &str) -> (i64, i64) {
    let parts: Vec<&str> = line.split(", ").collect();
    let x = parts[0].split("+").nth(1).unwrap().trim().parse().unwrap();
    let y = parts[1].split("+").nth(1).unwrap().trim().parse().unwrap();
    (x, y)
}

fn parse_prize(line: &str) -> (i64, i64) {
    let parts: Vec<&str> = line.split(", ").collect();
    let x = parts[0].split("=").nth(1).unwrap().trim().parse().unwrap();
    let y = parts[1].split("=").nth(1).unwrap().trim().parse().unwrap();
    (x, y)
}

fn solve_machine(machine: &Machine, check_limit: bool) -> Option<(i64, i64)> {
    // We have two equations:
    // a_x * A + b_x * B = prize_x
    // a_y * A + b_y * B = prize_y

    // Multiply first equation by b_y and second by b_x to eliminate B
    // b_y * (a_x * A + b_x * B) = b_y * prize_x
    // b_x * (a_y * A + b_y * B) = b_x * prize_y

    // Subtract to eliminate B
    // b_y * a_x * A - b_x * a_y * A = b_y * prize_x - b_x * prize_y

    let denominator =
        machine.b_y as f64 * machine.a_x as f64 - machine.b_x as f64 * machine.a_y as f64;
    if denominator == 0.0 {
        return None;
    }

    let a = (machine.b_y as f64 * machine.prize_x as f64
        - machine.b_x as f64 * machine.prize_y as f64)
        / denominator;
    if a <= 0.0 || (check_limit && a >= 100.0) || (a - a.round()).abs() > 1e-10 {
        return None;
    }
    let a = a.round() as i64;

    // Solve for B using first equation
    let b = (machine.prize_x - machine.a_x * a) as f64 / machine.b_x as f64;
    if b <= 0.0 || (check_limit && b >= 100.0) || (b - b.round()).abs() > 1e-10 {
        return None;
    }
    let b = b.round() as i64;

    Some((a, b))
}

fn solve(input: &Input) -> (usize, usize) {
    let mut total_tokens_part1 = 0;
    let mut total_tokens_part2 = 0;

    for machine in &input.machines {
        // Part 1: Original coordinates with 100 press limit
        if let Some((a, b)) = solve_machine(machine, true) {
            total_tokens_part1 += a * 3 + b;
        }

        // Part 2: Add 10000000000000 to prize coordinates, no press limit
        let machine_part2 = Machine {
            prize_x: machine.prize_x + 10000000000000,
            prize_y: machine.prize_y + 10000000000000,
            ..*machine
        };
        if let Some((a, b)) = solve_machine(&machine_part2, false) {
            total_tokens_part2 += a * 3 + b;
        }
    }

    (total_tokens_part1 as usize, total_tokens_part2 as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part1() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).0, 480);
        assert_eq!(solve(&input).1, 875318608908);
    }
}
