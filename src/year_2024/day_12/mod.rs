use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use anyhow::Result;

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

struct Map {
    dim: Dimension,
    grid: Vec<Vec<char>>,
}

#[derive(Clone, Copy)]
struct Dimension {
    height: usize,
    width: usize,
}

#[derive(Eq, PartialEq, Debug)]
struct Area {
    plots: HashSet<Position>,
}

impl Area {
    fn contains(&self, pos: Position) -> bool {
        self.plots.contains(&pos)
    }

    fn size(&self) -> usize {
        self.plots.len()
    }
}

fn parse_map(input: &str) -> Map {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let dim = Dimension {
        height: grid.len(),
        width: grid[0].len(),
    };

    Map { grid, dim }
}

fn count_open_sides(pos: Position, map: &Map) -> usize {
    let potential_neighbours = get_next_positions(pos, map.dim);
    let neighbour_count = potential_neighbours
        .iter()
        .filter(|n_pos| map.grid[n_pos.y][n_pos.x] == map.grid[pos.y][pos.x])
        .count();

    4 - neighbour_count
}

fn collect_areas(map: &Map) -> Result<HashMap<char, Vec<Area>>> {
    let mut result: HashMap<char, Vec<Area>> = HashMap::new();
    for (row_idx, row) in map.grid.iter().enumerate() {
        for (col_idx, plant) in row.iter().enumerate() {
            let pos = Position {
                x: col_idx,
                y: row_idx,
            };
            let position_already_found = result
                .entry(*plant)
                .or_default()
                .iter()
                .any(|area| area.contains(pos));

            if position_already_found {
                continue;
            }

            let area = discover_area(pos, map);
            result.entry(*plant).or_default().push(area);
        }
    }

    Ok(result)
}

fn discover_area(start_pos: Position, map: &Map) -> Area {
    let plot = map.grid[start_pos.y][start_pos.x];
    let mut positions = HashSet::new();

    breadth_first_search(plot, &mut positions, &HashSet::from([start_pos]), map);

    Area { plots: positions }
}

fn breadth_first_search(
    plot: char,
    found_positions: &mut HashSet<Position>,
    next_to_explore: &HashSet<Position>,
    map: &Map,
) {
    if next_to_explore.is_empty() {
        return;
    }

    let neighbours = next_to_explore
        .iter()
        .flat_map(|pos| get_next_positions(*pos, map.dim))
        .filter(|pos| !found_positions.contains(pos))
        .filter(|pos| map.grid[pos.y][pos.x] == plot)
        .collect::<HashSet<_>>();

    found_positions.extend(next_to_explore);

    breadth_first_search(plot, found_positions, &neighbours, map);
}

fn get_next_positions(start_pos: Position, dim: Dimension) -> HashSet<Position> {
    let mut positions = HashSet::new();

    if start_pos.x > 0 {
        positions.insert(Position {
            x: start_pos.x - 1,
            y: start_pos.y,
        });
    }

    if start_pos.y > 0 {
        positions.insert(Position {
            x: start_pos.x,
            y: start_pos.y - 1,
        });
    }

    if start_pos.x + 1 < dim.width {
        positions.insert(Position {
            x: start_pos.x + 1,
            y: start_pos.y,
        });
    }

    if start_pos.y + 1 < dim.height {
        positions.insert(Position {
            x: start_pos.x,
            y: start_pos.y + 1,
        });
    }

    positions
}

#[cfg(test)]
mod tests {
    use crate::year_2024::day_12::{get_next_positions, Dimension, Position};

    #[test]
    fn next_positions_correct() {
        let pos = Position { x: 0, y: 0 };
        let neighbours = get_next_positions(
            pos,
            Dimension {
                height: 2,
                width: 2,
            },
        );
        assert_eq!(
            neighbours,
            [Position { x: 1, y: 0 }, Position { x: 0, y: 1 }]
                .into_iter()
                .collect()
        );
    }
}
