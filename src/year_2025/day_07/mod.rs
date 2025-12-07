use std::collections::HashSet;

use macros::aoc_solver;

use anyhow::{anyhow, Result};

use crate::common::{
    math_2d::{Grid, UPoint},
    parsing,
};

static INPUT: &str = include_str!("input.txt");

#[derive(PartialEq)]
enum Tile {
    Empty,
    Splitter,
    Start,
}

#[aoc_solver(2025, 7, 1, INPUT)]
fn solve_part_1(input: &str) -> Result<String> {
    let grid = parse_input(input)?;
    let start_pos = grid
        .find_value(Tile::Start)
        .ok_or(anyhow!("Input does not contain the starting position"))?;
    let mut hit_splitters = HashSet::new();
    let mut current_beams = HashSet::new();
    current_beams.insert(start_pos);

    loop {
        let mut next_beam_set: HashSet<UPoint> = HashSet::new();
        for beam_pos in current_beams.iter() {
            let Some(splitter_pos) = find_splitter_pos(&grid, *beam_pos) else {
                continue;
            };

            hit_splitters.insert(splitter_pos);

            if let Some(left_beam) = splitter_pos.checked_sub(UPoint::new(1, 0)) {
                next_beam_set.insert(left_beam);
            }
            if let Some(right_beam) = splitter_pos.checked_add(UPoint::new(1, 0)) {
                next_beam_set.insert(right_beam);
            }
        }

        if next_beam_set.is_empty() {
            break;
        }

        current_beams = next_beam_set;
    }

    Ok(hit_splitters.len().to_string())
}

fn find_splitter_pos(grid: &Grid<Tile>, beam: UPoint) -> Option<UPoint> {
    for y in beam.y..grid.udims().height {
        let point = UPoint::new(beam.x, y);
        let tile = grid.get(point)?;
        if *tile == Tile::Splitter {
            return Some(point);
        }
    }

    None
}

fn parse_input(input: &str) -> Result<Grid<Tile>> {
    let mapper = |c| {
        if c == '.' {
            Ok(Tile::Empty)
        } else if c == '^' {
            Ok(Tile::Splitter)
        } else if c == 'S' {
            Ok(Tile::Start)
        } else {
            Err(anyhow!("Invalid character {}", c))
        }
    };
    let grid = parsing::parse_grid(input, mapper)?;
    Ok(grid)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example_part_1() {
        let result = super::solve_part_1(include_str!("example.txt")).unwrap();
        assert_eq!(result, "21");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve_part_1(super::INPUT).unwrap();
        assert_eq!(result, "1546");
    }
}
