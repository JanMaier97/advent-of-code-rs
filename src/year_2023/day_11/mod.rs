use std::collections::HashSet;

use crate::{print_challenge_header, MyResult};

const INPUT: &str = include_str!("input.txt");

pub fn solve() -> MyResult<()> {
    print_challenge_header(11);

    println!("Sum of all distances: {}", solve_part_one(INPUT));
    println!("Sum of all distances: {}", solve_part_two(INPUT));

    Ok(())
}

type Position = (usize, usize);

struct PuzzleInput {
    galaxies: HashSet<Position>,
}

fn solve_part_one(input: &str) -> usize {
    calculate_distance_sum(input, 2)
}

fn solve_part_two(input: &str) -> usize {
    calculate_distance_sum(input, 1_000_000)
}

fn calculate_distance_sum(input: &str, expansion_rate: usize) -> usize {
    let puzzle = parse_input(input, expansion_rate);
    sum_distances(&puzzle)
}

fn sum_distances(puzzle: &PuzzleInput) -> usize {
    let mut total_distance = 0;
    let mut points_to_ignore = HashSet::new();

    for galaxy in puzzle.galaxies.iter() {
        points_to_ignore.insert(galaxy);
        for other_galaxy in puzzle.galaxies.iter() {
            if points_to_ignore.contains(&other_galaxy) {
                continue;
            }
            total_distance += calculate_distance(&galaxy, &other_galaxy);
        }
    }

    total_distance
}

fn calculate_distance(pos_a: &Position, pos_b: &Position) -> usize {
    let x_difference = pos_a.0.abs_diff(pos_b.0);
    let y_difference = pos_a.1.abs_diff(pos_b.1);

    x_difference + y_difference
}

fn parse_input(input: &str, expansion_rate: usize) -> PuzzleInput {
    let empty_columns = parse_empty_columns(input);
    let empty_rows = parse_empty_rows(input);
    let galaxies = parse_galaxies(input, &empty_columns, &empty_rows, expansion_rate);

    PuzzleInput { galaxies }
}

fn parse_galaxies(
    input: &str,
    empty_columns: &HashSet<usize>,
    empty_rows: &HashSet<usize>,
    expansion_rate: usize,
) -> HashSet<Position> {
    let mut galaxies = HashSet::new();
    for (line_idx, line) in input.lines().enumerate() {
        for (char_idx, char) in line.chars().enumerate() {
            if char != '#' {
                continue;
            }

            let pos = adjust_position(
                (char_idx, line_idx),
                empty_columns,
                empty_rows,
                expansion_rate,
            );

            galaxies.insert(pos);
        }
    }

    galaxies
}

fn adjust_position(
    pos: Position,
    empty_columns: &HashSet<usize>,
    empty_rows: &HashSet<usize>,
    expansion_rate: usize,
) -> Position {
    let x_offset = (0..=pos.0).filter(|x| empty_columns.contains(&x)).count();
    let y_offset = (0..=pos.1).filter(|y| empty_rows.contains(&y)).count();

    (
        pos.0 + (x_offset * (expansion_rate - 1)),
        pos.1 + (y_offset * (expansion_rate - 1)),
    )
}

fn parse_empty_rows(input: &str) -> HashSet<usize> {
    input
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.contains('#'))
        .map(|(idx, _)| idx)
        .collect::<HashSet<_>>()
}

fn parse_empty_columns(input: &str) -> HashSet<usize> {
    let max_x = input.lines().take(1).last().unwrap().chars().count();

    let mut columns_with_galaxies = HashSet::new();
    for line in input.lines() {
        let galaxies_x_pos = line
            .chars()
            .enumerate()
            .filter(|(idx, c)| *c == '#')
            .map(|(idx, _)| idx);

        columns_with_galaxies.extend(galaxies_x_pos);
    }

    (0..max_x)
        .filter(|x| !columns_with_galaxies.contains(x))
        .collect::<HashSet<_>>()
}

#[cfg(test)]
mod tests {
    use crate::year_2023::day_11::{calculate_distance_sum, solve_part_two, INPUT};

    use super::solve_part_one;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    pub fn solve_part_one_example_correctly() {
        let result = solve_part_one(EXAMPLE);
        assert_eq!(result, 374);
    }

    #[test]
    pub fn solve_part_one_input_correctly() {
        let result = solve_part_one(INPUT);
        assert_eq!(result, 9724940);
    }

    #[test]
    pub fn solve_part_two_example_correctly_with_rate_10() {
        let result = calculate_distance_sum(EXAMPLE, 10);
        assert_eq!(result, 1030);
    }

    #[test]
    pub fn solve_part_two_example_correctly_with_rate_100() {
        let result = calculate_distance_sum(EXAMPLE, 100);
        assert_eq!(result, 8410);
    }

    #[test]
    pub fn solve_part_two_input_correctly() {
        let result = solve_part_two(INPUT);
        assert_eq!(result, 569052586852);
    }
}
