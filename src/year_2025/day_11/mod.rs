use std::collections::{HashMap, HashSet};

use macros::aoc_solver;

use anyhow::Result;

static INPUT: &str = include_str!("input.txt");

#[aoc_solver(2025, 11, 1, INPUT)]
fn solve_part_1(input: &str) -> Result<String> {
    let mapping = parse_input(input);
    let count = get_path_count("you", &mapping, &mut HashMap::new());
    Ok(count.to_string())
}

#[aoc_solver(2025, 11, 2, INPUT)]
fn solve_part_2(input: &str) -> Result<String> {
    let graph = parse_input(input);
    let reachable_map = build_reachable_mapping(&graph);
    let mut count_mapping = HashMap::new();
    get_path_count("svr", &graph, &mut count_mapping);
    let mut on_path = HashSet::new();
    let mut cache = HashMap::new();

    let count = get_problematic_path_count(
        "svr",
        &graph,
        &mut on_path,
        &reachable_map,
        &count_mapping,
        &mut cache,
    );

    Ok(count.to_string())
}

fn get_problematic_path_count(
    current_device: &str,
    graph: &HashMap<String, HashSet<String>>,
    nodes_on_path: &mut HashSet<String>,
    reachable_mapping: &HashMap<String, HashSet<String>>,
    count_mapping: &HashMap<String, u64>,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if current_device == "out" {
        let count = if nodes_on_path.contains("dac") && nodes_on_path.contains("fft") {
            1
        } else {
            0
        };

        return count;
    }

    if let Some(count) = cache.get(current_device) {
        return *count;
    }

    let dac_already_reached = nodes_on_path.contains("dac") || current_device == "dac";
    let fft_already_reached = nodes_on_path.contains("fft") || current_device == "fft";

    if dac_already_reached && fft_already_reached {
        return *count_mapping.get(current_device).unwrap();
    }

    if let Some(reachable) = reachable_mapping.get(current_device) {
        let dac_reachable = reachable.contains("dac") || dac_already_reached;
        let fft_reachable = reachable.contains("fft") || fft_already_reached;
        if !dac_reachable || !fft_reachable {
            return 0;
        }
    };

    let outputs = graph.get(current_device).unwrap();
    let mut total_count = 0;
    for next_device in outputs {
        nodes_on_path.insert(next_device.to_string());
        // current_path.push(next_device.to_string());
        let count = get_problematic_path_count(
            next_device,
            graph,
            nodes_on_path,
            reachable_mapping,
            count_mapping,
            cache,
            // current_path,
        );

        nodes_on_path.remove(next_device);
        // current_path.pop();
        total_count += count;
    }

    if total_count != 0 {
        cache.insert(current_device.to_string(), total_count);
    }

    total_count
}

fn build_reachable_mapping(
    graph: &HashMap<String, HashSet<String>>,
) -> HashMap<String, HashSet<String>> {
    let mut mapping = HashMap::new();
    dfs_collect_reachables("svr", graph, &mut mapping);
    mapping
}

fn dfs_collect_reachables(
    current_device: &str,
    graph: &HashMap<String, HashSet<String>>,
    mapping: &mut HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    if current_device == "out" {
        return HashSet::new();
    }

    if let Some(cached_result) = mapping.get(current_device) {
        return cached_result.clone();
    }

    let outputs = graph.get(current_device).unwrap();
    let mut result = outputs.clone();
    for next_device in outputs.iter().cloned() {
        let reachable = dfs_collect_reachables(&next_device, graph, mapping);
        result.extend(reachable.iter().cloned());
        mapping.insert(next_device, reachable);
    }

    result
}

fn get_path_count(
    current_device: &str,
    graph: &HashMap<String, HashSet<String>>,
    count_mapping: &mut HashMap<String, u64>,
) -> u64 {
    if current_device == "out" {
        return 1;
    }

    if let Some(count) = count_mapping.get(current_device) {
        return *count;
    }

    let outputs = graph.get(current_device).unwrap();
    let mut count = 0;
    for output in outputs {
        let c = get_path_count(output, graph, count_mapping);
        count += c;
        count_mapping.insert(output.to_string(), c);
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
    fn solve_example_part_2() {
        let result = super::solve_part_2(include_str!("example_2.txt")).unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve_part_1(super::INPUT).unwrap();
        assert_eq!(result, "652");
    }

    #[test]
    fn solve_part_2() {
        let result = super::solve_part_2(super::INPUT).unwrap();
        assert_eq!(result, "362956369749210");
    }
}
