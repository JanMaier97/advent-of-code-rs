use macros::aoc_solver;

use anyhow::Result;

use crate::year_2024::day_20::INPUT;

use super::{count_shortcuts_by_time_saved, parse_input};


#[aoc_solver(2024, 20, 1, INPUT)]
fn solve(input: &str) -> Result<String> {
    let input = parse_input(input)?;

    let counts = count_shortcuts_by_time_saved(&input);

    let sum: usize = counts
        .into_iter()
        .filter(|(saved, _)| *saved >= 100)
        .map(|(_, count)| count)
        .sum();

    Ok(sum.to_string())
}


#[cfg(test)]
mod tests {
    use crate::year_2024::day_20::{count_shortcuts_by_time_saved, parse_input};

    #[test]
    fn finds_correct_shortcuts() {
        let input = parse_input(include_str!("example.txt")).unwrap();
        let counts = count_shortcuts_by_time_saved(&input);

        assert_eq!(counts.get(&2).unwrap(), &14);
        assert_eq!(counts.get(&4).unwrap(), &14);
        assert_eq!(counts.get(&6).unwrap(), &2);
        assert_eq!(counts.get(&8).unwrap(), &4);
        assert_eq!(counts.get(&10).unwrap(), &2);
        assert_eq!(counts.get(&12).unwrap(), &3);
        assert_eq!(counts.get(&20).unwrap(), &1);
        assert_eq!(counts.get(&36).unwrap(), &1);
        assert_eq!(counts.get(&38).unwrap(), &1);
        assert_eq!(counts.get(&40).unwrap(), &1);
        assert_eq!(counts.get(&64).unwrap(), &1);
    }
}
