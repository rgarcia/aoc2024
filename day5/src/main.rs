use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

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
    // Read the file content
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let sections: Vec<&str> = content.split("\n\n").collect();

    // Parse ordering rules
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in sections[0].lines() {
        let parts: Vec<u32> = line.split('|').map(|s| s.parse().unwrap()).collect();
        rules
            .entry(parts[0])
            .or_insert_with(HashSet::new)
            .insert(parts[1]);
    }

    // Check each update
    let mut sum_of_middle_pages = 0;
    for line in sections[1].lines() {
        let pages: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        if is_correct_order(&pages, &rules) {
            let middle_index = pages.len() / 2;
            sum_of_middle_pages += pages[middle_index];
        }
    }

    println!("{}", sum_of_middle_pages);
}

fn is_correct_order(pages: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    for (i, &page) in pages.iter().enumerate() {
        for &next_page in &pages[i + 1..] {
            // Check if current page must come after next_page
            if let Some(after_pages) = rules.get(&next_page) {
                if after_pages.contains(&page) {
                    return false;
                }
            }
        }
    }
    true
}

fn part2(file_path: &str) {
    // Read the file content
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let sections: Vec<&str> = content.split("\n\n").collect();

    // Parse ordering rules
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in sections[0].lines() {
        let parts: Vec<u32> = line.split('|').map(|s| s.parse().unwrap()).collect();
        rules
            .entry(parts[0])
            .or_insert_with(HashSet::new)
            .insert(parts[1]);
    }

    // Check each update and reorder if necessary
    let mut sum_of_middle_pages = 0;
    for line in sections[1].lines() {
        let pages: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        if !is_correct_order(&pages, &rules) {
            let sorted_pages = topological_sort(&pages, &rules);
            let middle_index = sorted_pages.len() / 2;
            sum_of_middle_pages += sorted_pages[middle_index];
        }
    }

    println!("{}", sum_of_middle_pages);
}

fn topological_sort(pages: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    let mut in_degree = HashMap::new();
    let mut graph = HashMap::new();

    // Initialize graph and in-degree count
    for &page in pages {
        in_degree.entry(page).or_insert(0);
        graph.entry(page).or_insert_with(HashSet::new);
    }

    // Build graph and in-degree count
    for &page in pages {
        if let Some(after_pages) = rules.get(&page) {
            for &after_page in after_pages {
                if pages.contains(&after_page) {
                    graph.get_mut(&page).unwrap().insert(after_page);
                    *in_degree.entry(after_page).or_insert(0) += 1;
                }
            }
        }
    }

    // Topological sort using Kahn's algorithm
    let mut sorted = Vec::new();
    let mut queue: Vec<u32> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&page, _)| page)
        .collect();

    while let Some(page) = queue.pop() {
        sorted.push(page);
        if let Some(neighbors) = graph.get(&page) {
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push(neighbor);
                    }
                }
            }
        }
    }

    sorted
}
