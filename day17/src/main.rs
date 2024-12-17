use aochelpers::get_daily_input;
use std::error::Error;

struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    ip: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

struct Input {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    program: Vec<u8>,
}

impl Computer {
    fn new(input: &Input) -> Self {
        Computer {
            reg_a: input.reg_a,
            reg_b: input.reg_b,
            reg_c: input.reg_c,
            ip: 0,
            program: input.program.clone(),
            output: Vec::new(),
        }
    }

    fn get_combo_value(&self, operand: u8) -> i64 {
        match operand {
            0..=3 => operand as i64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("Invalid combo operand 7"),
            _ => panic!("Invalid combo operand > 7"),
        }
    }

    fn step(&mut self) -> bool {
        if self.ip >= self.program.len() - 1 {
            return false;
        }

        let opcode = self.program[self.ip];
        let operand = self.program[self.ip + 1];

        match opcode {
            0 => {
                let divisor = 1 << self.get_combo_value(operand);
                self.reg_a /= divisor;
            }
            1 => {
                self.reg_b ^= operand as i64;
            }
            2 => {
                self.reg_b = self.get_combo_value(operand) % 8;
            }
            3 => {
                if self.reg_a != 0 {
                    self.ip = operand as usize;
                    return true;
                }
            }
            4 => {
                self.reg_b ^= self.reg_c;
            }
            5 => {
                let value = (self.get_combo_value(operand) % 8) as u8;
                self.output.push(value);
            }
            6 => {
                let divisor = 1 << self.get_combo_value(operand);
                self.reg_b = self.reg_a / divisor;
            }
            7 => {
                let divisor = 1 << self.get_combo_value(operand);
                self.reg_c = self.reg_a / divisor;
            }
            _ => panic!("Invalid opcode"),
        }

        self.ip += 2;
        true
    }

    fn run(&mut self) {
        while self.step() {}
    }

    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(17, 2024)?;
    let input = parse_data(&data);

    let start = std::time::Instant::now();
    let part1 = solve_part1(&input);
    let duration1 = start.elapsed();

    let start = std::time::Instant::now();
    let part2 = solve_part2(&input);
    let duration2 = start.elapsed();

    println!("Part 1: {} ({} µs)", part1, duration1.as_micros());
    println!("Part 2: {} ({} µs)", part2, duration2.as_micros());

    Ok(())
}

fn parse_data(input: &str) -> Input {
    let mut lines = input.lines();

    // Parse register values
    let reg_a = lines
        .next()
        .unwrap()
        .trim_start_matches("Register A: ")
        .parse()
        .unwrap();
    let reg_b = lines
        .next()
        .unwrap()
        .trim_start_matches("Register B: ")
        .parse()
        .unwrap();
    let reg_c = lines
        .next()
        .unwrap()
        .trim_start_matches("Register C: ")
        .parse()
        .unwrap();

    // Skip empty line
    lines.next();

    // Parse program
    let program_str = lines.next().unwrap().trim_start_matches("Program: ");
    let program: Vec<u8> = program_str.split(',').map(|n| n.parse().unwrap()).collect();

    Input {
        reg_a,
        reg_b,
        reg_c,
        program,
    }
}

fn solve_part1(input: &Input) -> String {
    let mut computer = Computer::new(input);
    computer.run();
    computer.get_output()
}

fn solve_part2(input: &Input) -> i64 {
    let program = &input.program;
    let mut a: i64 = 0;

    for n in 1..=program.len() {
        let target = &program[program.len() - n..];
        let mut new_a = a << 3;

        loop {
            let mut test_computer = Computer::new(&Input {
                reg_a: new_a,
                reg_b: input.reg_b,
                reg_c: input.reg_c,
                program: input.program.clone(),
            });

            test_computer.run();

            let output = test_computer.output;
            if output.len() >= n && output[output.len() - n..] == target[..] {
                a = new_a;
                break;
            }
            new_a += 1;
        }
    }
    a
}

fn solve(input: &Input) -> (String, i64) {
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    // const TESTDATA: &str = "Register A: 729
    // Register B: 0
    // Register C: 0

    // Program: 0,1,5,4,3,0";

    const TESTDATA2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    //
    // #[test]
    // fn test_part1() {
    //     let input = parse_data(TESTDATA);
    //     assert_eq!(solve(&input).0, "4,6,3,5,6,3,5,2,1,0");
    // }

    #[test]
    fn test_part2() {
        let input = parse_data(TESTDATA2);
        assert_eq!(solve(&input).1, 117440);
    }
}
