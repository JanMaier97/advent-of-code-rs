use std::collections::HashSet;

use macros::aoc_solver;

use anyhow::{anyhow, Context, Result};

static INPUT: &str = include_str!("input.txt");

type LightIndex = u8;
type BitPattern = u64;

struct MachineData {
    light_pattern: BitPattern,
    wirings: Vec<BitPattern>,
    joltages: Vec<u32>,
}

#[aoc_solver(2025, 10, 1, INPUT)]
fn solve_part_1(input: &str) -> Result<String> {
    let machines = parse_input(input)?;
    let result: u64 = machines.iter().map(|m| get_required_presses(&m)).sum();
    return Ok(result.to_string());
}

fn get_required_presses(machine: &MachineData) -> u64 {
    let mut start_patterns = HashSet::from_iter([0]);
    button_sequence_bfs(&mut start_patterns, &machine, 0)
}

fn button_sequence_bfs(
    current_patterns: &mut HashSet<BitPattern>,
    machine: &MachineData,
    current_depth: u64,
) -> u64 {
    if current_patterns.iter().any(|l| *l == machine.light_pattern) {
        return current_depth;
    }

    let mut next_lights = HashSet::new();
    for wiring in machine.wirings.iter() {
        for light in current_patterns.iter() {
            let new_light = light ^ wiring;
            if current_patterns.contains(&new_light) {
                continue;
            }
            next_lights.insert(new_light);
        }
    }

    return button_sequence_bfs(&mut next_lights, machine, current_depth + 1);
}

fn create_bit_pattern(lights: &HashSet<u8>, light_count: usize) -> BitPattern {
    let mut id = 0;
    let mut mask = 1_u64;
    for index in (0..light_count).rev() {
        if lights.contains(&(index as LightIndex)) {
            id += mask;
        }
        mask <<= 1_u64;
    }

    id
}

fn parse_input(input: &str) -> Result<Vec<MachineData>> {
    let mut machines = Vec::new();
    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<_>>();

        let (light_pattern, light_count) = parse_lights(&parts)?;
        let m = MachineData {
            light_pattern,
            wirings: parse_wirings(&parts, light_count)?,
            joltages: parse_voltages(&parts)?,
        };
        machines.push(m);
    }

    Ok(machines)
}

fn parse_voltages(split_line: &[&str]) -> Result<Vec<u32>> {
    Ok(Vec::new())
}

fn parse_wirings(split_line: &[&str], light_count: usize) -> Result<Vec<BitPattern>> {
    let patterns = split_line
        .iter()
        .skip(1)
        .take(split_line.len() - 2)
        .map(|schema| {
            schema
                .replace('(', "")
                .replace(')', "")
                .split(',')
                .map(|c| {
                    c.parse::<LightIndex>()
                        .with_context(|| format!("Failed to parse the wiring schema {}", schema))
                })
                .collect::<Result<HashSet<_>>>()
        })
        .map(|res| res.map(|indeces| create_bit_pattern(&indeces, light_count)))
        .collect::<Result<Vec<_>>>()?;
    Ok(patterns)
}

fn parse_lights(split_line: &[&str]) -> Result<(BitPattern, usize)> {
    let light_indeces = split_line
        .first()
        .ok_or_else(|| anyhow!("Failed to parse light diagram"))?
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| {
            if c == '#' {
                Some(idx as LightIndex - 1)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    let light_count = split_line.first().unwrap().len() - 2;

    Ok((create_bit_pattern(&light_indeces, light_count), light_count))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn solve_example_part_1() {
        let result = super::solve_part_1(include_str!("example.txt")).unwrap();
        assert_eq!(result, "7");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve_part_1(super::INPUT).unwrap();
        assert_eq!(result, "494");
    }

    #[test]
    fn create_bitpattern_for_lighs() {
        let res = super::create_bit_pattern(&HashSet::from_iter([1, 2]), 4);
        assert_eq!(res, 0b0110);
        let res = super::create_bit_pattern(&HashSet::from_iter([3]), 5);
        assert_eq!(res, 0b0010);
        let res = super::create_bit_pattern(&HashSet::from_iter([1, 2, 3, 5]), 6);
        assert_eq!(res, 0b011101);
    }
}
