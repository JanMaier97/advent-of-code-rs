use itertools::Itertools;
use macros::aoc_solver;

use super::{parse_input, Emulator};

use anyhow::Result;

#[aoc_solver(2024, 17, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let rom = parse_input(input)?;

    let mut emu = Emulator::from_rom(rom);
    let output = emu.run()?;

    Ok(output.iter().join(","))
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn solve_input() {
        let result = super::solve(include_str!("input.txt")).unwrap();
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }
}
