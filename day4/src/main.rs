use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <part1|part2> <file_path>", args[0]);
        std::process::exit(1);
    }

    let command = &args[1];
    let file_path = &args[2];

    let start = std::time::Instant::now();
    match command.as_str() {
        "part1" => part1(file_path),
        "part2" => part2(file_path),
        _ => {
            eprintln!("Invalid command: {}. Use 'part1' or 'part2'.", command);
            std::process::exit(1);
        }
    }
    let duration = start.elapsed();
    eprintln!("Time: {:.6} seconds", duration.as_secs_f64());
}

fn part1(file_path: &str) {
    let input = std::fs::read_to_string(file_path).expect("Failed to read file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let sum = count_xmas(&grid);
    println!("{}", sum);
}

fn count_xmas(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // diagonal down-right
        (-1, 1),  // diagonal up-right
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // diagonal up-left
        (1, -1),  // diagonal down-left
    ];

    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            for &(dx, dy) in &directions {
                if check_xmas(grid, row, col, dx, dy) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn check_xmas(grid: &Vec<Vec<char>>, row: usize, col: usize, dx: i32, dy: i32) -> bool {
    let word = "XMAS";
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    for (i, expected_char) in word.chars().enumerate() {
        let new_row = row as i32 + dx * i as i32;
        let new_col = col as i32 + dy * i as i32;

        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            return false;
        }

        if grid[new_row as usize][new_col as usize] != expected_char {
            return false;
        }
    }
    true
}

fn part2(file_path: &str) {
    let input = std::fs::read_to_string(file_path).expect("Failed to read file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut sum = 0;

    // valid patterns correspond to top-left, top-right, bottom-left, bottom-right
    const VALID_PATTERNS: [(char, char, char, char); 4] = [
        ('M', 'M', 'S', 'S'),
        ('S', 'M', 'S', 'M'),
        ('S', 'S', 'M', 'M'),
        ('M', 'S', 'M', 'S'),
    ];

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if grid[row][col] != 'A' {
                continue;
            }

            let corners = [
                grid[row - 1][col - 1], // top-left
                grid[row - 1][col + 1], // top-right
                grid[row + 1][col - 1], // bottom-left
                grid[row + 1][col + 1], // bottom-right
            ];

            if VALID_PATTERNS.contains(&(corners[0], corners[1], corners[2], corners[3])) {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}
