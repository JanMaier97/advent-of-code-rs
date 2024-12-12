use macros::aoc_solver;

use crate::MyResult;

use super::{collect_areas, count_open_sides, parse_map, Area, Map};

#[aoc_solver(2024, 12, 1, super::INPUT)]
fn solve(input: &str) -> MyResult<u64> {
    let map = parse_map(input);
    let areas = collect_areas(&map)?;

    let sum: usize = areas
        .iter()
        .flat_map(|(_, areas)| areas.iter().map(|a| get_perimeter(a, &map) * a.size()))
        .sum();

    Ok(sum.try_into()?)
}

fn get_perimeter(area: &Area, map: &Map) -> usize {
    area.plots
        .iter()
        .map(|pos| count_open_sides(*pos, map))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::year_2024::day_12::{collect_areas, parse_map};

    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, 1930);
    }

    #[test]
    fn solve_small_example() {
        let input = "AAAA\nBBCD\nBBCC\nEEEC";
        let result = super::solve(input).unwrap();
        assert_eq!(result, 140);
    }

    #[test]
    fn solve_example_with_multiple_small_areas() {
        let input = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO";
        let result = super::solve(input).unwrap();
        assert_eq!(result, 720);
    }

    #[test]
    fn small_test() {
        let res = super::solve("A").unwrap();
        assert_eq!(res, 4);

        let res = super::solve("AA").unwrap();
        assert_eq!(res, 12);
    }

    #[test]
    fn permiter_correct() {
        let map = parse_map("AA\nAB");
        let plant_ares = collect_areas(&map).unwrap();

        let areas = plant_ares.get(&'A').unwrap();

        assert_eq!(areas.len(), 1, "areas is wrong: {:?}", areas);
        assert_eq!(areas[0].size(), 3);
        assert_eq!(super::get_perimeter(&areas[0], &map), 8);
    }
}
