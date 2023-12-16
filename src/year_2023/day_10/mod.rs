use std::{
    collections::{HashMap, HashSet},
    str::Chars,
};

use itertools::Itertools;

use crate::{print_challenge_header, MyResult};

const INPUT: &str = include_str!("input.txt");

pub fn solve() -> MyResult<()> {
    print_challenge_header(10);
    println!(
        "The step count to the farthest point is {}",
        solve_part_one(INPUT)
    );
    println!("{}", solve_part_two(INPUT));

    Ok(())
}

fn solve_part_one(input: &str) -> usize {
    let mut map = parse_input(input);
    let (start_pos, start_tile_type) = determine_start_tile(&map);

    map.insert(start_pos.clone(), start_tile_type.clone());

    let loop_tiles = get_loop_tiles(&map, Tile::new(start_pos, start_tile_type));
    let steps = loop_tiles.len() / 2;

    steps
}

fn solve_part_two(input: &str) -> usize {
    let mut map = parse_input(input);
    let (start_pos, start_tile_type) = determine_start_tile(&map);

    map.insert(start_pos.clone(), start_tile_type.clone());

    let loop_tiles = get_loop_tiles(&map, Tile::new(start_pos, start_tile_type));
    let double_horizontals = get_doubled_pipes(&map);

    for tile in loop_tiles
        .iter()
        .filter(|t| !double_horizontals.contains(t))
    {
        map.remove(&tile.pos);
    }

    let tiles = map
        .iter()
        .map(|(pos, ttype)| Tile::new(pos.clone(), *ttype))
        .collect_vec();

    let mut external_tiles: HashSet<Tile> = HashSet::new();
    let mut enclosed_tiles: HashSet<Tile> = HashSet::new();
    let (max_x, max_y) = get_map_size(&map);

    for tile in tiles {
        if external_tiles.contains(&tile) || enclosed_tiles.contains(&tile) {
            continue;
        }

        let reachable_tiles = traverse_breadth_first_enclosed_tiles(&map, &tile);

        if reachable_tiles.iter().any(|t| {
            t.pos.x == 0
                || t.pos.y == 0
                || t.pos.x == max_x
                || t.pos.y == max_y
                || external_tiles.contains(t)
        }) {
            external_tiles.extend(reachable_tiles);
        } else {
            enclosed_tiles.extend(reachable_tiles);
        }
    }

    for doubled_horizontal in double_horizontals {
        println!(
            "removing horizontal: {:?}: {}",
            doubled_horizontal,
            enclosed_tiles.contains(&doubled_horizontal)
        );
        enclosed_tiles.remove(&doubled_horizontal);
    }

    print_map(&map, &enclosed_tiles, &external_tiles);

    enclosed_tiles.len()
}

fn get_doubled_pipes(map: &HashMap<Position, TileType>) -> HashSet<Tile> {
    let mut x = get_doubled_tile(map, TileType::Horizontal, TileType::Vertical);
    let y = get_doubled_tile(map, TileType::Vertical, TileType::Horizontal);

    x.extend(y);
    x
}

fn get_doubled_tile(
    map: &HashMap<Position, TileType>,
    direction: TileType,
    opposite_direction: TileType,
) -> HashSet<Tile> {
    map.iter()
        .filter(|(_, &tt)| tt == direction)
        .flat_map(|(pos, _)| {
            opposite_direction
                .get_adjacent_positions(pos, map)
                .into_iter()
                .filter(|p| *map.get(p).unwrap() == direction)
        })
        .map(|pos| Tile::new(pos.clone(), *map.get(&pos).unwrap()))
        .collect::<HashSet<_>>()
}

fn traverse_breadth_first_enclosed_tiles(
    map: &HashMap<Position, TileType>,
    start: &Tile,
) -> HashSet<Tile> {
    let mut tiles_to_visit = HashSet::from([start.clone()]);
    let mut visited_tiles = HashSet::new();

    loop {
        let mut next_tiles = HashSet::new();
        for tile in tiles_to_visit.iter() {
            let neighbor_tiles = get_positions_for_enclosed_tiles(map, &tile.pos)
                .into_iter()
                .filter(|t| !visited_tiles.contains(t));
            next_tiles.extend(neighbor_tiles);
        }

        visited_tiles.extend(tiles_to_visit.clone());
        tiles_to_visit = next_tiles;

        if tiles_to_visit.is_empty() {
            break;
        }
    }

    visited_tiles
}

fn get_positions_for_enclosed_tiles(
    map: &HashMap<Position, TileType>,
    pos: &Position,
) -> HashSet<Tile> {
    // let offsets = [
    //     (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)
    // ];
    let mut offsets = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    if false {
        offsets.push((-1, 1))
    }

    offsets
        .iter()
        .map(|(x, y)| Position::new(pos.x + x, pos.y + y))
        .filter(|pos| map.contains_key(pos))
        .map(|pos| Tile::new(pos.clone(), *map.get(&pos).unwrap()))
        .collect::<HashSet<_>>()
}

fn get_loop_tiles(map: &HashMap<Position, TileType>, start: Tile) -> HashSet<Tile> {
    let mut visited_tiles: HashSet<Tile> = HashSet::new();
    let mut tiles_to_visit = HashSet::from([start]);

    loop {
        let mut discovered_positions = HashSet::new();

        for tile in tiles_to_visit.iter() {
            let neighbors = get_adjacent_tiles(tile, &map)
                .into_iter()
                .filter(|t| !visited_tiles.contains(t));

            discovered_positions.extend(neighbors);
        }

        visited_tiles.extend(tiles_to_visit.into_iter());
        tiles_to_visit = discovered_positions.clone();

        if tiles_to_visit.is_empty() {
            break;
        }
    }

    visited_tiles
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Tile {
    pos: Position,
    r#type: TileType,
}

impl Tile {
    fn new(pos: Position, r#type: TileType) -> Self {
        Self { pos, r#type }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum TileType {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToEast,
    SouthToWest,
    Start,
    Ground,
}

impl TileType {
    fn get_adjacent_positions(
        &self,
        pos: &Position,
        map: &HashMap<Position, TileType>,
    ) -> Vec<Position> {
        let positions = match self {
            TileType::Vertical => vec![
                Position::new(pos.x, pos.y - 1),
                Position::new(pos.x, pos.y + 1),
            ],
            TileType::Horizontal => vec![
                Position::new(pos.x + 1, pos.y),
                Position::new(pos.x - 1, pos.y),
            ],
            TileType::NorthToEast => vec![
                Position::new(pos.x, pos.y - 1),
                Position::new(pos.x + 1, pos.y),
            ],
            TileType::NorthToWest => vec![
                Position::new(pos.x, pos.y - 1),
                Position::new(pos.x - 1, pos.y),
            ],
            TileType::SouthToEast => vec![
                Position::new(pos.x, pos.y + 1),
                Position::new(pos.x + 1, pos.y),
            ],
            TileType::SouthToWest => vec![
                Position::new(pos.x, pos.y + 1),
                Position::new(pos.x - 1, pos.y),
            ],
            TileType::Start => vec![
                Position::new(pos.x, pos.y - 1),
                Position::new(pos.x, pos.y + 1),
                Position::new(pos.x - 1, pos.y),
                Position::new(pos.x + 1, pos.y),
            ],
            TileType::Ground => Vec::new(),
        };

        positions
            .into_iter()
            .filter(|pos| map.contains_key(pos))
            .collect_vec()
    }

    fn to_char(&self) -> char {
        match self {
            TileType::Vertical => '|',
            TileType::Horizontal => '-',
            TileType::NorthToEast => 'L',
            TileType::NorthToWest => 'J',
            TileType::SouthToWest => '7',
            TileType::SouthToEast => 'F',
            TileType::Ground => '.',
            TileType::Start => 'S',
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '|' => TileType::Vertical,
            '-' => TileType::Horizontal,
            'L' => TileType::NorthToEast,
            'J' => TileType::NorthToWest,
            '7' => TileType::SouthToWest,
            'F' => TileType::SouthToEast,
            '.' => TileType::Ground,
            'S' => TileType::Start,
            _ => panic!(),
        }
    }
}

fn determine_start_tile(map: &HashMap<Position, TileType>) -> (Position, TileType) {
    let (start_pos, start_tile_type) = map
        .iter()
        .filter(|(_, tile)| **tile == TileType::Start)
        .last()
        .unwrap();

    let surrounding_positions = start_tile_type.get_adjacent_positions(start_pos, &map);

    let connected_positions = surrounding_positions
        .into_iter()
        .filter(|pos| {
            map.get(pos)
                .unwrap()
                .get_adjacent_positions(pos, &map)
                .contains(start_pos)
        })
        .collect_vec();

    assert_eq!(connected_positions.len(), 2);

    let has_north = connected_positions.contains(&Position::new(start_pos.x, start_pos.y - 1));
    let has_south = connected_positions.contains(&Position::new(start_pos.x, start_pos.y + 1));
    let has_east = connected_positions.contains(&Position::new(start_pos.x + 1, start_pos.y));
    let has_west = connected_positions.contains(&Position::new(start_pos.x - 1, start_pos.y));

    let tile_type = match (has_north, has_south, has_east, has_west) {
        (true, true, false, false) => TileType::Vertical,
        (true, false, true, false) => TileType::NorthToEast,
        (true, false, false, true) => TileType::NorthToWest,
        (false, true, true, false) => TileType::SouthToEast,
        (false, true, false, true) => TileType::SouthToWest,
        (false, false, true, true) => TileType::Horizontal,
        _ => panic!(
            "Invalid configuration ({},{},{},{})",
            has_north, has_south, has_east, has_west
        ),
    };

    (start_pos.clone(), tile_type)
}

fn parse_input(input: &str) -> HashMap<Position, TileType> {
    input
        .lines()
        .enumerate()
        .flat_map(|(idx, line)| parse_line(idx, line.chars()))
        .collect::<HashMap<_, _>>()
}

fn parse_line(line_index: usize, input: Chars) -> Vec<(Position, TileType)> {
    input
        .enumerate()
        .map(|(idx, c)| {
            (
                Position::new(idx as i32, line_index as i32),
                TileType::from_char(c),
            )
        })
        .collect_vec()
}

fn get_adjacent_tiles(tile: &Tile, map: &HashMap<Position, TileType>) -> Vec<Tile> {
    let positions = tile.r#type.get_adjacent_positions(&tile.pos, map);

    positions
        .into_iter()
        .map(|pos| Tile::new(pos.clone(), *map.get(&pos).unwrap()))
        .collect_vec()
}

fn get_map_size(map: &HashMap<Position, TileType>) -> (i32, i32) {
    map.keys().map(|pos| (pos.x, pos.y)).max().unwrap()
}

fn print_map(
    map: &HashMap<Position, TileType>,
    enclosed_tiles: &HashSet<Tile>,
    outclosed_tiles: &HashSet<Tile>,
) {
    let dimension = get_map_size(map);
    for y in 0..=dimension.1 {
        let mut line = Vec::new();
        for x in 0..=dimension.0 {
            let pos = Position::new(x, y);
            if let Some(t) = map.get(&pos) {
                if enclosed_tiles.iter().any(|t| t.pos == pos) {
                    line.push('I');
                } else if outclosed_tiles.iter().any(|t| t.pos == pos) {
                    line.push('O');
                } else {
                    line.push(t.to_char());
                }
            } else {
                line.push('#');
            };
        }
        println!("{}", line.iter().collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::year_2023::{
        day_10::INPUT,
        day_10::{solve_part_two, Position, TileType},
    };

    use super::{determine_start_tile, parse_input, solve_part_one};

    const EXAMPLE1_INPUT: &str = include_str!("example1.txt");
    const EXAMPLE2_INPUT: &str = include_str!("example2.txt");
    const PART_2_EXAMPLE1_INPUT: &str = include_str!("part2_example1.txt");
    const PART_2_EXAMPLE2_INPUT: &str = include_str!("part2_example2.txt");
    const PART_2_EXAMPLE3_INPUT: &str = include_str!("part2_example3.txt");
    const PART_2_EXAMPLE4_INPUT: &str = include_str!("part2_example4.txt");

    #[test]
    fn start_north_to_south_correct() {
        let input = "|\nS\n|";
        let map = parse_input(input);
        let (pos, tile_type) = determine_start_tile(&map);
        assert_eq!(pos, Position::new(0, 1));
        assert_eq!(tile_type, TileType::Vertical);
    }

    #[test]
    fn start_east_to_west_correct() {
        let input = "-S-";
        let map = parse_input(input);
        let (pos, tile_type) = determine_start_tile(&map);
        assert_eq!(pos, Position::new(1, 0));
        assert_eq!(tile_type, TileType::Horizontal);
    }

    #[test]
    fn start_north_to_east_correct() {
        let input = "|\nS-";
        let map = parse_input(input);
        let (pos, tile_type) = determine_start_tile(&map);
        assert_eq!(pos, Position::new(0, 1));
        assert_eq!(tile_type, TileType::NorthToEast);
    }

    #[test]
    fn start_in_example1_correct() {
        let map = parse_input(EXAMPLE1_INPUT);
        let (pos, tile_type) = determine_start_tile(&map);
        assert_eq!(pos, Position::new(1, 1));
        assert_eq!(tile_type, TileType::SouthToEast);
    }

    #[test]
    fn neighboring_positions_correct() {
        use TileType as TT;
        let start_pos = Position::new(0, 0);
        let tiles = [
            (TT::Horizontal, ((-1, 0), (1, 0))),
            (TT::Vertical, ((0, -1), (0, 1))),
            (TT::NorthToEast, ((0, -1), (1, 0))),
            (TT::NorthToWest, ((0, -1), (-1, 0))),
            (TT::SouthToEast, ((0, 1), (1, 0))),
            (TT::SouthToWest, ((0, 1), (-1, 0))),
        ];

        let mut map = HashMap::new();
        map.insert(Position::new(-1, 0), TT::Ground);
        map.insert(Position::new(1, 0), TT::Ground);
        map.insert(Position::new(0, 1), TT::Ground);
        map.insert(Position::new(0, -1), TT::Ground);

        for (tile_type, ((x1, y1), (x2, y2))) in tiles {
            let positions = tile_type.get_adjacent_positions(&start_pos, &map);

            assert!(
                positions.contains(&Position::new(x1, y1)),
                "{:?} - {:?} does not contain ({}, {})",
                tile_type,
                positions,
                x1,
                y1,
            );
            assert!(
                positions.contains(&Position::new(x2, y2)),
                "{:?} - {:?} does not contain ({}, {})",
                tile_type,
                positions,
                x2,
                y2,
            );
        }
    }

    #[test]
    fn solve_example1_part_one_correctly() {
        let result = solve_part_one(EXAMPLE1_INPUT);
        assert_eq!(result, 4);
    }

    #[test]
    fn solve_example2_part_one_correctly() {
        let result = solve_part_one(EXAMPLE2_INPUT);
        assert_eq!(result, 8);
    }

    #[test]
    fn solve_real_part_one_correctly() {
        let result = solve_part_one(INPUT);
        assert_eq!(result, 6907);
    }

    #[test]
    fn solve_example1_part_two_correctly() {
        let result = solve_part_two(PART_2_EXAMPLE1_INPUT);
        assert_eq!(result, 4);
    }

    #[test]
    fn solve_example2_part_two_correctly() {
        let result = solve_part_two(PART_2_EXAMPLE2_INPUT);
        assert_eq!(result, 4);
    }

    #[test]
    fn solve_example3_part_two_correctly() {
        let result = solve_part_two(PART_2_EXAMPLE3_INPUT);
        assert_eq!(result, 8);
    }

    #[test]
    fn solve_example4_part_two_correctly() {
        let result = solve_part_two(PART_2_EXAMPLE4_INPUT);
        assert_eq!(result, 10);
    }
}
