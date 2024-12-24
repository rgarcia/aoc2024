use aochelpers::get_daily_input;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

struct Input<'a> {
    wire_states: HashMap<&'a str, bool>,
    operations: HashMap<&'a str, (&'a str, &'a str, &'a str)>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(24, 2024)?;

    let input = parse_data(&data);
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    let dot = create_dot(&input);
    std::fs::write("circuit.dot", dot)?;

    Ok(())
}

fn parse_data(input: &str) -> Input {
    let (s1, s2) = input.split_once("\n\n").unwrap();
    let mut wire_states = HashMap::new();
    let mut operations = HashMap::new();
    for l in s1.lines() {
        let (n, v) = l.split_once(": ").unwrap();
        wire_states.insert(n, v == "1");
    }
    for l in s2.lines() {
        let (a, op, b, _, c) = l.split_whitespace().collect_tuple().unwrap();
        operations.insert(c, (a, op, b));
    }
    Input {
        wire_states,
        operations,
    }
}

fn solve(input: &Input) -> (usize, String) {
    let mut wire_states = input.wire_states.clone();
    let mut changed = true;

    // Keep applying operations until no more changes occur
    while changed {
        changed = false;
        for (output, &(in1, op, in2)) in &input.operations {
            // Skip if output already has a value
            if wire_states.contains_key(output) {
                continue;
            }

            // Try to get input values
            if let (Some(&val1), Some(&val2)) = (wire_states.get(in1), wire_states.get(in2)) {
                let result = match op {
                    "AND" => val1 & val2,
                    "OR" => val1 | val2,
                    "XOR" => val1 ^ val2,
                    _ => unreachable!(),
                };
                wire_states.insert(*output, result);
                changed = true;
            }
        }
    }

    // Calculate result from z-wires
    let mut result = 0;
    let mut power = 0;

    // Collect all z-wires and sort them by number
    let mut z_wires: Vec<_> = wire_states
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .collect();
    z_wires.sort_by_key(|(k, _)| k.to_string());

    // Build the result number
    for (_, &value) in z_wires {
        if value {
            result |= 1 << power;
        }
        power += 1;
    }

    // New part 2 logic: Find broken nodes
    let mut broken_nodes = HashSet::new();

    // Build edges map (how many times each wire is used as input)
    let mut edges: HashMap<&str, usize> = HashMap::new();
    for (_, (in1, _, in2)) in &input.operations {
        *edges.entry(in1).or_default() += 1;
        *edges.entry(in2).or_default() += 1;
    }

    for (output, (in1, op, in2)) in &input.operations {
        // z nodes must be XOR (except for the last one)
        if output.starts_with("z") && *output != "z45" && *op != "XOR" {
            broken_nodes.insert(*output);
        }

        // z nodes must not be inputs of other nodes
        if in1.starts_with("z") {
            broken_nodes.insert(in1);
        }
        if in2.starts_with("z") {
            broken_nodes.insert(in2);
        }

        // inputs of XOR nodes (except for z nodes) must be x and y nodes
        if *op == "XOR"
            && !output.starts_with("z")
            && !((in1.starts_with("x") && in2.starts_with("y"))
                || (in1.starts_with("y") && in2.starts_with("x")))
        {
            broken_nodes.insert(*output);
        }

        // XOR nodes (except z nodes) must always be input of exactly two other nodes
        if *op == "XOR" && !output.starts_with("z") && edges.get(output).copied().unwrap_or(0) != 2
        {
            broken_nodes.insert(*output);
        }

        // AND nodes must always be input of exactly one other node
        // (except the very first one wired to x00 and y00)
        if *op == "AND"
            && !output.starts_with("z")
            && edges.get(output).copied().unwrap_or(0) != 1
            && !((*in1 == "x00" && *in2 == "y00") || (*in1 == "y00" && *in2 == "x00"))
        {
            broken_nodes.insert(*output);
        }
    }
    let mut broken_nodes = broken_nodes.into_iter().collect::<Vec<_>>();
    broken_nodes.sort();

    (result, broken_nodes.join(","))
}

fn create_dot(input: &Input) -> String {
    let mut dot = String::from("digraph circuit {\n");

    // Add nodes for initial wire states
    for (wire, &state) in &input.wire_states {
        dot.push_str(&format!(
            "    {} [label=\"{} ({})\"];\n",
            wire,
            wire,
            if state { "1" } else { "0" }
        ));
    }

    // Add nodes and edges for operations
    for (output, (in1, op, in2)) in &input.operations {
        // Color based on operation
        let color = match *op {
            "AND" => "lightblue",
            "OR" => "lightgreen",
            "XOR" => "pink",
            _ => "white",
        };

        // Create operation node
        let op_node = format!("{}_op", output);
        dot.push_str(&format!(
            "    {} [label=\"{}\" shape=box style=filled fillcolor=\"{}\"];\n",
            op_node, op, color
        ));

        // Add edges
        dot.push_str(&format!("    {} -> {};\n", in1, op_node));
        dot.push_str(&format!("    {} -> {};\n", in2, op_node));
        dot.push_str(&format!("    {} -> {};\n", op_node, output));
    }

    dot.push_str("}\n");
    dot
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const TESTDATA2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_part1() {
        let input = parse_data(TESTDATA);
        assert_eq!(solve(&input).0, 4);
    }

    #[test]
    fn test_part2() {
        let input = parse_data(TESTDATA2);
        assert_eq!(solve(&input).0, 2024);
    }
}
