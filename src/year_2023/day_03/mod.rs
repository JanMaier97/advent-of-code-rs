use std::collections::HashSet;

use crate::{MyResult, print_challenge_header};

const INPUT: &str = include_str!("input.txt");

type Position = (usize, usize);
type SymbolPositions = HashSet<Position>;

#[derive(Debug)]
struct NumberPosition {
    pos: Position,
    len: usize,
    value: u32,
}

pub fn solve() -> MyResult<()> {
    print_challenge_header(3);

    println!("The sum of all part numbers is: {}", solve_part_one(INPUT));
    println!("XXX: {}", solve_part_two(INPUT));

    Ok(())
}


fn solve_part_one(input: &str) -> u32 {
    let (numbers, symbols) = parse_input(input);

    let mut sum = 0;
    for number in numbers {
        let adjacent_positions = get_adjacent_positions(&number);
        if adjacent_positions.iter().any(|pos| symbols.contains(pos)) {
            sum += number.value;
            continue;
        }

    }
    return sum;

}

fn solve_part_two(input: &str) -> u32 {
    unimplemented!()
}

fn get_adjacent_positions(number: &NumberPosition) -> Vec<Position> {
    let mut positions = Vec::new();

    let (start_x, start_y) = number.pos;

    // column before number
    if start_x > 0 {
        let prev_x = start_x-1;
        positions.push((prev_x, start_y));
        positions.push((prev_x, start_y+1));
        if start_y > 0 {
            positions.push((prev_x, start_y-1));
        }
    }

    // column after number
    let post_x = start_x + number.len;
    positions.push((post_x, start_y));
    positions.push((post_x, start_y+1));
    if start_y > 0 {
        positions.push((post_x, start_y-1));
    }

    for index in 0..number.len {
        let inner_x = start_x + index;
        positions.push((inner_x, start_y + 1));
        if start_y > 0 {
            positions.push((inner_x, start_y - 1));
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
                    value: adjacent_digits.iter().collect::<String>().parse::<u32>().unwrap(),
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

    if char == '.'  {
        return false;
    }
    
    true
}



#[cfg(test)]
mod tests {
    use crate::year_2023::day_03::INPUT;

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
}