use std::collections::HashMap;

use macros::aoc_solver;

use anyhow::Result;

use super::parse_input;

#[aoc_solver(2024, 1, 2, super::INPUT)]
pub fn solve(input: &str) -> Result<String> {
    let locations = parse_input(input)?;
    let frequencies = get_frequencies(&locations.list2);
    let result: u32 = locations
        .list1
        .into_iter()
        .map(|id| id * frequencies.get(&id).unwrap_or(&0))
        .sum();
    Ok(result.to_string())
}

fn get_frequencies(location_ids: &[u32]) -> HashMap<u32, u32> {
    let mut result = HashMap::new();
    location_ids
        .iter()
        .for_each(|id| *result.entry(*id).or_default() += 1);
    result
}

#[cfg(test)]
mod tests {
    use crate::year_2024::day_01::part_2::solve;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn solve_example() {
        let result = solve(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "31");
    }
}
