use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{print_challenge_header, MyResult};

const INPUT: &str = include_str!("input.txt");

type Position = (usize, usize);
type SymbolPositions = HashSet<Position>;

#[derive(Debug)]
struct NumberPosition {
    pos: Position,
    len: usize,
    value: u32,
}

impl NumberPosition {
    fn get_adjacent_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();

        let (start_x, start_y) = self.pos;

        // column before number
        if start_x > 0 {
            let prev_x = start_x - 1;
            positions.push((prev_x, start_y));
            positions.push((prev_x, start_y + 1));
            if start_y > 0 {
                positions.push((prev_x, start_y - 1));
            }
        }

        // column after number
        let post_x = start_x + self.len;
        positions.push((post_x, start_y));
        positions.push((post_x, start_y + 1));
        if start_y > 0 {
            positions.push((post_x, start_y - 1));
        }

        for index in 0..self.len {
            let inner_x = start_x + index;
            positions.push((inner_x, start_y + 1));
            if start_y > 0 {
                positions.push((inner_x, start_y - 1));
            }
        }
        positions
    }

    fn is_part_number(&self, symbols: &SymbolPositions) -> bool {
        let adjacent_positions = self.get_adjacent_positions();
        return adjacent_positions.iter().any(|pos| symbols.contains(pos));
    }

    fn get_positions(&self) -> Vec<Position> {
        let (pos_x, pos_y) = self.pos;

        let positions = (0..self.len).map(|index| (pos_x + index, pos_y)).collect();

        positions
    }
}

pub fn solve() -> MyResult<()> {
    print_challenge_header(3);

    println!("The sum of all part numbers is: {}", solve_part_one(INPUT));
    println!("The total gear power is: {}", solve_part_two(INPUT));

    Ok(())
}

fn solve_part_one(input: &str) -> u32 {
    let (numbers, symbols) = parse_input(input);

    let sum = numbers
        .iter()
        .filter(|n| n.is_part_number(&symbols))
        .map(|n| n.value)
        .sum();

    return sum;
}

fn solve_part_two(input: &str) -> u32 {
    let (numbers, symbols) = parse_input(input);

    let part_numbers_positions = numbers
        .iter()
        .filter(|n| n.is_part_number(&symbols))
        .flat_map(|n| n.get_positions().into_iter().map(|pos| (pos, n.value)))
        .collect::<HashMap<_, _>>();

    let sum = symbols
        .into_iter()
        .map(|pos| calculate_gear_value(&pos, &part_numbers_positions))
        .sum();

    sum
}

fn calculate_gear_value(symbol_pos: &Position, part_numbers: &HashMap<Position, u32>) -> u32 {
    let adjacent_positions = get_adjacent_positions(symbol_pos);
    let mut neighboring_numbers = adjacent_positions
        .iter()
        .map(|pos| part_numbers.get(pos).copied())
        .flatten()
        .collect_vec();

    // silent bug:
    // if a symbol has exactly 2 adjacent numbers with the same value
    // then the symbol is not registered as a gear
    neighboring_numbers.sort();
    neighboring_numbers.dedup();

    if neighboring_numbers.len() == 2 {
        return neighboring_numbers
            .into_iter()
            .fold(1, |acc, value| acc * value);
    }

    return 0;
}

fn get_adjacent_positions(pos: &Position) -> Vec<Position> {
    let (center_x, center_y) = *pos;
    let min_x = center_x.saturating_sub(1);
    let min_y = center_y.saturating_sub(1);
    let max_x = center_x + 1;
    let max_y = center_y + 1;

    let mut positions = Vec::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if x == center_x && y == center_y {
                continue;
            }
            positions.push((x, y));
        }
    }

    positions
}

fn parse_input(input: &str) -> (Vec<NumberPosition>, SymbolPositions) {
    let numbers = parse_number_positions(input);
    let symbols = parse_symbol_positions(input);

    (numbers, symbols)
}

fn parse_number_positions(input: &str) -> Vec<NumberPosition> {
    let mut numbers = Vec::new();
    for (line_number, line) in input.lines().enumerate() {
        let mut adjacent_digits: Vec<char> = Vec::new();
        let mut current_pos: Option<Position> = None;

        let last_char_index = line.len() - 1;
        for (char_index, char) in line.chars().enumerate() {
            if char.is_numeric() {
                adjacent_digits.push(char);

                if current_pos.is_none() {
                    current_pos = Some((char_index, line_number));
                }
            }

            if current_pos.is_some() && (!char.is_numeric() || char_index == last_char_index) {
                let number = NumberPosition {
                    pos: current_pos.unwrap(),
                    len: adjacent_digits.len(),
                    value: adjacent_digits
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap(),
                };

                numbers.push(number);
                current_pos = None;
                adjacent_digits.clear();
            }
        }
    }

    numbers
}

fn parse_symbol_positions(input: &str) -> SymbolPositions {
    let mut symbol_positions = HashSet::new();

    for (line_number, line) in input.lines().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            if is_symbol(char) {
                symbol_positions.insert((char_index, line_number));
            }
        }
    }

    symbol_positions
}

fn is_symbol(char: char) -> bool {
    if char.is_alphanumeric() {
        return false;
    }

    if char == '.' {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::year_2023::day_03::{solve_part_two, INPUT};

    use super::solve_part_one;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn part_one_example_solved_correctly() {
        let result = solve_part_one(EXAMPLE_INPUT);
        assert_eq!(result, 4361)
    }

    #[test]
    fn part_one_input_solved_correctly() {
        let result = solve_part_one(INPUT);
        assert_eq!(result, 537832)
    }

    #[test]
    fn part_two_example_solved_correctly() {
        let result = solve_part_two(EXAMPLE_INPUT);
        assert_eq!(result, 467835)
    }

    #[test]
    fn part_two_input_solved_correctly() {
        let result = solve_part_two(INPUT);
        assert_eq!(result, 81939900)
    }
}
