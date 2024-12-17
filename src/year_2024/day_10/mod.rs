use std::collections::HashSet;

use anyhow::Result;

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy, PartialEq)]
struct Dimensions {
    height: usize,
    width: usize,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

struct PuzzleInput {
    start_positions: HashSet<Position>,
    map: Map,
}

struct Map {
    grid: Vec<Vec<u8>>,
    dims: Dimensions,
}

fn get_next_positions(pos: Position, map: &Map) -> HashSet<Position> {
    let mut positions = HashSet::new();
    let expected_value = map.grid[pos.y][pos.x] + 1;
    if expected_value > 9 {
        return positions;
    }

    let x = pos.x;
    let y = pos.y;

    if pos.x > 0 && map.grid[y][x - 1] == expected_value {
        positions.insert(Position { x: x - 1, y });
    }

    if pos.x < map.dims.width - 1 && map.grid[y][x + 1] == expected_value {
        positions.insert(Position { x: x + 1, y });
    }

    if pos.y > 0 && map.grid[y - 1][x] == expected_value {
        positions.insert(Position { x, y: y - 1 });
    }

    if pos.y < map.dims.height - 1 && map.grid[y + 1][x] == expected_value {
        positions.insert(Position { x, y: y + 1 });
    }

    positions
}

fn parse_input(input: &str) -> Result<PuzzleInput> {
    let mut grid = Vec::new();
    let mut start_positions = HashSet::new();
    for (line_idx, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (char_idx, char) in line.chars().enumerate() {
            let value = char.to_string().parse::<u8>()?;
            if value == 0 {
                start_positions.insert(Position {
                    x: char_idx,
                    y: line_idx,
                });
            }
            row.push(value);
        }
        grid.push(row);
    }

    let dims = Dimensions {
        height: grid.len(),
        width: grid[0].len(),
    };

    let map = Map { grid, dims };

    let puzzle_input = PuzzleInput {
        map,
        start_positions,
    };

    Ok(puzzle_input)
}
