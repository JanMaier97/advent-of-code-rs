use std::collections::{HashMap, HashSet};

use macros::aoc_solver;

use anyhow::Result;

use crate::{
    common::math_2d::{Grid, Point, PointIdx, Vec2},
    year_2024::day_20::INPUT,
};

use super::{parse_input, Tile};

#[aoc_solver(2024, 20, 2, INPUT)]
fn solve(input: &str) -> Result<String> {
    let input = parse_input(input)?;

    let counts = count_shortcuts(&input.grid, &input.path, 20);

    let sum: usize = counts
        .into_iter()
        .filter(|(saved, _)| *saved >= 100)
        .map(|(_, count)| count)
        .sum();

    Ok(sum.to_string())
}

fn count_shortcuts(
    grid: &Grid<Tile>,
    path: &[Point<i32>],
    max_length: usize,
) -> HashMap<usize, usize> {
    let index_map = path
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, p)| (p, idx))
        .collect::<HashMap<_, _>>();
    let mut already_found: HashSet<(Point<i32>, Point<i32>)> = HashSet::new();
    let mut count_map = HashMap::new();

    for start in path.iter().cloned() {
        let shortcut_ends = find_shortcuts_for_point(start, grid, &index_map, max_length);
        for (end, saved) in shortcut_ends {
            if !already_found.contains(&(end, start)) {
                already_found.insert((start, end));
                count_map.entry(saved).and_modify(|c| *c += 1).or_insert(1);
            }
        }
    }

    count_map
}

fn find_shortcuts_for_point(
    start: Point<i32>,
    grid: &Grid<Tile>,
    index_map: &HashMap<Point<i32>, usize>,
    distance: usize,
) -> HashSet<(Point<i32>, usize)> {
    let reachable_points = get_free_points_within_distance(start, grid, distance);

    let mut actual_shortcuts = HashSet::new();
    for end in reachable_points {
        let shortcut_distance = manhatten(start, end);
        let actual_distance = index_map
            .get(&start)
            .unwrap()
            .abs_diff(*index_map.get(&end).unwrap());
        if shortcut_distance < actual_distance {
            actual_shortcuts.insert((end, actual_distance - shortcut_distance));
        }
    }

    actual_shortcuts
}

fn manhatten(a: Point<i32>, b: Point<i32>) -> usize {
    (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)) as usize
}

fn get_free_points_within_distance(
    center: Point<i32>,
    grid: &Grid<Tile>,
    distance: usize,
) -> HashSet<Point<i32>> {
    let distance = distance as i32;
    let mut points = HashSet::new();

    for y in 0..=distance {
        for x in 0..=(distance - y) {
            if y == 0 && x == 0 {
                continue;
            }
            points.extend([
                center + Vec2::new(x, y),
                center + Vec2::new(-x, y),
                center + Vec2::new(x, -y),
                center + Vec2::new(-x, -y),
            ]);
        }
    }

    points
        .into_iter()
        .filter(|p| grid.get_at(*p).is_some_and(|t| *t != Tile::Wall))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::year_2024::day_20::{parse_input, part_2::count_shortcuts};

    #[test]
    fn finds_correct_shortcuts() {
        let input = parse_input(include_str!("example.txt")).unwrap();
        let counts = count_shortcuts(&input.grid, &input.path, 20);

        assert_eq!(counts.get(&50).unwrap(), &32);
        assert_eq!(counts.get(&52).unwrap(), &31);
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

    #[test]
    fn test_part_1_solutions() {
        let input = parse_input(include_str!("example.txt")).unwrap();
        let counts = count_shortcuts(&input.grid, &input.path, 2);

        println!("found shortcuts: ");
        println!("{:?}", counts);

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
