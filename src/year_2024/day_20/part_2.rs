use macros::aoc_solver;

use anyhow::Result;

use crate::year_2024::day_20::INPUT;

use super::{count_shortcuts_by_time_saved, parse_input};


#[aoc_solver(2024, 20, 2, INPUT)]
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

        assert_eq!(counts.get(&50).unwrap(), &32);
        assert_eq!(counts.get(&52).unwrap(), &21);
        assert_eq!(counts.get(&54).unwrap(), &29);
        assert_eq!(counts.get(&56).unwrap(), &39);
        assert_eq!(counts.get(&58).unwrap(), &25);
        assert_eq!(counts.get(&60).unwrap(), &23);
        assert_eq!(counts.get(&62).unwrap(), &20);
        assert_eq!(counts.get(&64).unwrap(), &19);
        assert_eq!(counts.get(&66).unwrap(), &12);
        assert_eq!(counts.get(&68).unwrap(), &14);
        assert_eq!(counts.get(&70).unwrap(), &12);
        assert_eq!(counts.get(&72).unwrap(), &22);
        assert_eq!(counts.get(&74).unwrap(), &4);
        assert_eq!(counts.get(&76).unwrap(), &3);
    }
}