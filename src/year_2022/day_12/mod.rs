use std::collections::HashSet;

use itertools::Itertools;

use crate::{print_challenge_header, MyResult};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Map {
    row_count: usize,
    col_count: usize,
    start_pos: (usize, usize),
    target_pos: (usize, usize),
    tiles: Vec<Vec<u8>>,
}

impl Map {
    fn new(input: &str) -> MyResult<Self> {
        // println!("new map..");
        let mut start_pos = None;
        let mut end_pos = None;
        let mut tiles = Vec::new();

        for (row_idx, line) in input.lines().enumerate() {
            if line.chars().any(|c| !c.is_ascii_alphabetic()) {
                return Err("Non ascii alphabetic char found".into());
            }

            tiles.push(
                line.chars()
                    .enumerate()
                    .map(|(col_idx, c)| match c {
                        'S' => {
                            start_pos = Some((row_idx, col_idx));
                            'a'
                        }
                        'E' => {
                            end_pos = Some((row_idx, col_idx));
                            'z'
                        }
                        _ => c,
                    })
                    .map(|c| c as u8 - 'a' as u8)
                    .collect_vec(),
            );
        }

        let Some(start_pos) = start_pos else {
            return Err("Map does not contain starting position".into());
        };

        let Some(end_pos) = end_pos else {
            return Err("Map does not contain end position".into());
        };

        let Some(first_row) = tiles.first() else {
            return Err("Empty map".into());
        };

        if tiles.iter().skip(1).any(|row| row.len() != first_row.len()) {
            return Err("Map has inconsistent row lengths".into());
        }

        Ok(Map {
            row_count: tiles.len(),
            col_count: first_row.len(),
            target_pos: end_pos,
            start_pos,
            tiles,
        })
    }

    fn get_neighbouring_tiles(&self, center: &(usize, usize)) -> HashSet<(usize, usize)> {
        let (row, col) = *center;
        let current_tile = self.tiles[row][col];

        let mut result = HashSet::new();
        // left
        if col > 0 && current_tile + 1 >= self.tiles[row][col - 1] {
            result.insert((row, col - 1));
        }

        // right
        if col < self.col_count - 1 && current_tile + 1 >= self.tiles[row][col + 1] {
            result.insert((row, col + 1));
        }

        // up
        if row > 0 && current_tile + 1 >= self.tiles[row - 1][col] {
            result.insert((row - 1, col));
        }

        // down
        if row < self.row_count - 1 && current_tile + 1 >= self.tiles[row + 1][col] {
            result.insert((row + 1, col));
        }

        return result;
    }

    pub fn find_path(&self) -> MyResult<usize> {
        // println!("finding path...");
        let tiles_to_visit = HashSet::from([self.start_pos]);
        let mut visited_tiles = HashSet::new();
        let path = Vec::new();

        return self.breadth_first_search(&tiles_to_visit, &mut visited_tiles, &path, 0);
    }

    pub fn breadth_first_search(
        &self,
        tiles_to_visit: &HashSet<(usize, usize)>,
        visited: &mut HashSet<(usize, usize)>,
        path: &Vec<(usize, usize)>,
        step: usize,
    ) -> MyResult<usize> {
        let mut next_to_visit: HashSet<(usize, usize)> = HashSet::new();

        if tiles_to_visit.is_empty() {
            return Err("Could not find path".into());
        }

        for tile in tiles_to_visit {
            if *tile == self.target_pos {
                return Ok(step);
            }

            let neighbours = self.get_neighbouring_tiles(&tile);

            next_to_visit.extend(
                neighbours
                    .iter()
                    .filter(|&t| !visited.contains(t) || !tiles_to_visit.contains(t)),
            );
        }

        visited.extend(tiles_to_visit);

        return self.breadth_first_search(&next_to_visit, visited, path, step + 1);
    }

    fn shortest_path_from_lowest_tiles(&self) -> MyResult<usize> {
        let mut starting_points = Vec::new();
        for row in 0..self.row_count {
            for col in 0..self.col_count {
                if self.tiles[row][col] == 0 {
                    starting_points.push((row, col));
                }
            }
        }

        let tiles_to_visit = HashSet::from_iter(starting_points);
        let mut visited_tiles = HashSet::new();
        let path = Vec::new();

        return self.breadth_first_search(&tiles_to_visit, &mut visited_tiles, &path, 0);
    }
}

pub fn solve() -> MyResult<()> {
    print_challenge_header(12);
    println!("Skipping for performance...");

    println!(
        "1) The fewest steps to the location is {}",
        solve_part_one(INPUT)?
    );
    println!(
        "2) The fewest steps to the location is {}",
        solve_part_two(INPUT)?
    );

    Ok(())
}

fn solve_part_one(input: &str) -> MyResult<u32> {
    let map = Map::new(input)?;
    let path = map.find_path()?;
    Ok(path as u32)
}

fn solve_part_two(input: &str) -> MyResult<u32> {
    let map = Map::new(input)?;
    let path = map.shortest_path_from_lowest_tiles()?;
    Ok(path as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn solve_part_one_example() {
        let result = solve_part_one(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 31);
    }

    #[test]
    fn solve_part_one_real() {
        let result = solve_part_one(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 472);
    }

    #[test]
    fn solve_part_two_example() {
        let result = solve_part_two(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 29);
    }

    #[test]
    fn solve_part_two_real() {
        let result = solve_part_two(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 465);
    }
}
