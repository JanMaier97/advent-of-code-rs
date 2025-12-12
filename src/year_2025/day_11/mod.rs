use std::collections::{HashMap, HashSet};

use macros::aoc_solver;

use anyhow::{bail, ensure, Context, Result};

static INPUT: &str = include_str!("input.txt");

#[aoc_solver(2025, 11, 1, INPUT)]
fn solve_part_1(input: &str) -> Result<String> {
    let mapping = parse_input(input);
    let count = get_path_count("you", &mapping);
    Ok(count.to_string())
}

fn get_path_count(current_device: &str, graph: &HashMap<String, HashSet<String>>) -> u32 {
    if current_device == "out" {
        return 1;
    }

    let outputs = graph.get(current_device).unwrap();
    let mut count = 0;
    for output in outputs {
        count += get_path_count(output, graph);
    }

    count
}

fn parse_input(input: &str) -> HashMap<String, HashSet<String>> {
    let mut mapping = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(' ');
        let device = split.next().unwrap().replace(':', "").to_string();
        let outputs = split.map(|s| s.to_string()).collect::<HashSet<_>>();
        mapping.insert(device, outputs);
    }

    mapping
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example_part_1() {
        let result = super::solve_part_1(include_str!("example.txt")).unwrap();
        assert_eq!(result, "5");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve_part_1(super::INPUT).unwrap();
        assert_eq!(result, "652");
    }
}
