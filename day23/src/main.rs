use aochelpers::get_daily_input;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

struct Input {
    adj_list: HashMap<String, HashSet<String>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(23, 2024)?;

    let input = parse_data(&data);
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_data(input: &str) -> Input {
    let mut adj_list = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        adj_list
            .entry(a.to_string())
            .or_insert_with(HashSet::new)
            .insert(b.to_string());
        adj_list
            .entry(b.to_string())
            .or_insert_with(HashSet::new)
            .insert(a.to_string());
    }

    Input { adj_list }
}

fn solve(input: &Input) -> (usize, String) {
    // TODO, model solve here
    (part1(input), part2(input))
}

fn part1(input: &Input) -> usize {
    let mut triangles = HashSet::new();

    // For each vertex u
    for (u, u_neighbors) in &input.adj_list {
        // For each neighbor v of u
        for v in u_neighbors {
            // Get neighbors of v
            let v_neighbors = input.adj_list.get(v).unwrap();

            // Find common neighbors between u and v
            for w in u_neighbors.intersection(v_neighbors) {
                // Create a sorted triangle representation for deduplication
                let mut triangle = vec![u.clone(), v.clone(), w.clone()];
                triangle.sort();

                // Only count if at least one vertex starts with 't'
                if triangle.iter().any(|x| x.starts_with('t')) {
                    triangles.insert(triangle);
                }
            }
        }
    }

    triangles.len()
}

fn part2(input: &Input) -> String {
    // Get vertices sorted by degree (descending)
    let mut vertices: Vec<&String> = input.adj_list.keys().collect();
    vertices.sort_by_key(|v| std::cmp::Reverse(input.adj_list[*v].len()));

    let mut max_clique = HashSet::new();

    for &vertex in &vertices {
        // Only proceed if this vertex could potentially lead to a larger clique
        if max_clique.len() < input.adj_list[vertex].len() + 1 {
            // Start with current vertex
            let mut current_clique = HashSet::new();
            current_clique.insert(vertex.clone());

            // Get its neighbors as candidates
            let mut candidates: HashSet<String> = input.adj_list[vertex].clone();

            // While we have candidates and could still beat max_clique
            while !candidates.is_empty()
                && current_clique.len() + candidates.len() > max_clique.len()
            {
                // Find highest degree vertex among candidates
                let next_vertex = candidates
                    .iter()
                    .max_by_key(|v| input.adj_list[*v].len())
                    .unwrap()
                    .clone();

                current_clique.insert(next_vertex.clone());

                // Update candidates to only include neighbors of all clique vertices
                candidates = candidates
                    .intersection(&input.adj_list[&next_vertex])
                    .cloned()
                    .collect();

                // Verify it's still a valid clique
                if !is_clique(&current_clique, &input.adj_list) {
                    break;
                }
            }

            if current_clique.len() > max_clique.len() {
                max_clique = current_clique;
            }
        }
    }

    // Format result as comma-separated string
    let mut result: Vec<String> = max_clique.into_iter().collect();
    result.sort();
    result.join(",")
}

fn is_clique(vertices: &HashSet<String>, adj_list: &HashMap<String, HashSet<String>>) -> bool {
    for v1 in vertices {
        for v2 in vertices {
            if v1 != v2 && !adj_list[v1].contains(v2) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part1() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).0, 7);
    }

    #[test]
    fn test_part2() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).1, "co,de,ka,ta");
    }
}
