use std::collections::HashMap;

use linkme::distributed_slice;

use crate::{MyResult, SolverMetadata, SOLVERS};

use super::parse_input;

#[distributed_slice(SOLVERS)]
static PART1_SOLVER: SolverMetadata<'static> = SolverMetadata {year: 2024, day: 1, part: 2, func: solve, input: super::INPUT };

pub fn solve(input: &str) -> MyResult<u32> {
    let locations = parse_input(input)?;
    let frequencies = get_frequencies(&locations.list2);
    let result = locations.list1.into_iter()
        .map(|id|  id * frequencies.get(&id).unwrap_or(&0))
        .sum();
    Ok(result)
}

fn get_frequencies(location_ids: &[u32]) -> HashMap<u32, u32> {
    let mut result = HashMap::new();
    location_ids.iter().for_each(
        |id| *result.entry(*id).or_default() += 1
    );
    result
}

#[cfg(test)]
mod tests {
    use crate::year_2024::day_01::part_2::solve;
    use crate::year_2024::day_01::INPUT;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn test_part_one_example() {
        let result = solve(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 31);
    }

    #[test]
    fn test_part_one_input() {
        let result = solve(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 23177084);
    }
}
