use std::collections::HashSet;

use macros::aoc_solver;

use anyhow::{anyhow, Context, Result};

static INPUT: &str = include_str!("input.txt");

type LightIndex = u8;
type LightId = u64;

struct MachineData {
    lights: HashSet<LightIndex>,
    light_pattern: LightId,
    light_count: usize,
    wirings: Vec<HashSet<LightIndex>>,
    joltages: Vec<u32>,
}

#[aoc_solver(2025, 10, 1, INPUT)]
fn solve_part_1(input: &str) -> Result<String> {
    let machines = parse_input(input)?;
    let result: u64 = machines.iter().map(|m| get_required_presses(&m)).sum();
    return Ok(result.to_string());
}

fn get_required_presses(machine: &MachineData) -> u64 {
    let mut start_lights = vec![HashSet::new()];
    let mut start_ids = HashSet::from_iter([0]);
    button_sequence_bfs(&mut start_lights, &mut start_ids, &machine, 0)
}

fn button_sequence_bfs(
    current_lights: &mut Vec<HashSet<LightIndex>>,
    current_light_ids: &mut HashSet<LightId>,
    machine: &MachineData,
    current_depth: u64,
) -> u64 {
    if current_light_ids
        .iter()
        .any(|l| *l == machine.light_pattern)
    {
        return current_depth;
    }

    // println!("goal: {}", generate_light_id(&machine.lights, machine.light_count));
    let mut next_lights = Vec::new();
    for wiring in machine.wirings.iter() {
        for light in current_lights.iter() {
            let new_light = wiring
                .union(light)
                .filter(|l| !(light.contains(l) && wiring.contains(l)))
                .cloned()
                .collect();
            let light_id = generate_light_id(&new_light, machine.light_count);
            // println!("light {:?} - wiring {:?} => {:?} - {}", light, wiring, new_light, light_id);
            if current_light_ids.contains(&light_id) {
                // println!("--skipped");
                continue;
            }

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("error: unable to read user input");
            next_lights.push(new_light);
            current_light_ids.insert(light_id);
        }
    }

    return button_sequence_bfs(
        &mut next_lights,
        current_light_ids,
        machine,
        current_depth + 1,
    );
}

fn generate_light_id(lights: &HashSet<LightIndex>, light_count: usize) -> LightId {
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

        let (lights, light_count) = parse_lights(&parts)?;
        let light_pattern = generate_light_id(&lights, light_count);
        let m = MachineData {
            lights,
            light_count,
            light_pattern,
            wirings: parse_wirings(&parts)?,
            joltages: parse_voltages(&parts)?,
        };
        machines.push(m);
    }

    Ok(machines)
}

fn parse_voltages(split_line: &[&str]) -> Result<Vec<u32>> {
    Ok(Vec::new())
}

fn parse_wirings(split_line: &[&str]) -> Result<Vec<HashSet<LightIndex>>> {
    let res = split_line
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
        .collect::<Result<Vec<_>>>()?;
    Ok(res)
}

fn parse_lights(split_line: &[&str]) -> Result<(HashSet<LightIndex>, usize)> {
    let active_lights = split_line
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

    let count = split_line.first().unwrap().len() - 2;

    Ok((active_lights, count))
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
        let res = super::generate_light_id(&HashSet::from_iter([1, 2]), 4);
        assert_eq!(res, 0b0110);
        let res = super::generate_light_id(&HashSet::from_iter([3]), 5);
        assert_eq!(res, 0b0010);
        let res = super::generate_light_id(&HashSet::from_iter([1, 2, 3, 5]), 6);
        assert_eq!(res, 0b011101);
    }
}
