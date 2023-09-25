use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use crate::{print_challange_header, MyResult};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub fn solve() -> MyResult<()> {
    print_challange_header(9);

    println!(
        "1) The number of position visited at least once is {}",
        solve_first_part(INPUT)?
    );
    println!(
        "2) The number of position visited at least once is {}",
        solve_second_part(INPUT)?
    );

    Ok(())
}

fn solve_first_part(input: &str) -> MyResult<u32> {
    let moves = parse_moves(input)?;

    let mut head_pos: (i32, i32) = (0, 0);
    let mut tail_pos: (i32, i32) = (0, 0);
    let mut visited_positions = HashSet::from([tail_pos]);

    for (direction, count) in moves {
        for _ in 0..count {
            head_pos = get_new_head_position(&direction, head_pos);

            tail_pos = get_tail_position(head_pos, tail_pos);

            visited_positions.insert(tail_pos);
        }
    }

    Ok(visited_positions.len() as u32)
}

fn get_tail_position(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    let difference = (head_pos.0 - tail_pos.0, head_pos.1 - tail_pos.1);
    if (difference.0).abs() > 1 || (difference.1).abs() > 1 {
        return (
            tail_pos.0 + difference.0.clamp(-1, 1),
            tail_pos.1 + difference.1.clamp(-1, 1),
        );
    }

    tail_pos
}

fn get_new_head_position(direction: &Direction, head_pos: (i32, i32)) -> (i32, i32) {
    match *direction {
        Direction::Up => (head_pos.0, head_pos.1 + 1),
        Direction::Down => (head_pos.0, head_pos.1 - 1),
        Direction::Left => (head_pos.0 - 1, head_pos.1),
        Direction::Right => (head_pos.0 + 1, head_pos.1),
    }
}

fn parse_moves(input: &str) -> MyResult<Vec<(Direction, u32)>> {
    let mut moves = Vec::new();
    for line in input.lines() {
        let Some((direction, count)) = line.split_once(' ') else {
            return Err("Invalid line".into());
        };

        let count = count.parse::<u32>()?;
        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => return Err("Invalid direction".into()),
        };

        moves.push((direction, count));
    }
    Ok(moves)
}

fn solve_second_part(input: &str) -> MyResult<u32> {
    let moves = parse_moves(input)?;

    let mut tail_positions = vec![(0i32, 0i32); 10];
    let mut visited_positions = HashSet::from([*tail_positions.last().unwrap()]);

    for (direction, count) in moves {
        for _ in 0..count {
            // set head
            tail_positions[0] = get_new_head_position(&direction, tail_positions[0]);

            // adjust tails
            for i in 1..tail_positions.len() {
                tail_positions[i] = get_tail_position(tail_positions[i - 1], tail_positions[i]);
            }

            visited_positions.insert(*tail_positions.last().unwrap());
        }
    }

    Ok(visited_positions.len() as u32)
}

fn print_board(knots: &[(i32, i32)]) {
    for row in (0..5i32).rev() {
        for col in 0..6i32 {
            let Some((index, _)) = knots.iter().find_position(|&&knot| knot == (col, row)) else {
                print!(".");
                continue;
            };
            print!("{}", index);
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("example1.txt");
    const EXAMPLE_INPUT_2: &str = include_str!("example2.txt");

    #[test]
    fn solve_first_part_example() {
        let result = solve_first_part(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 13);
    }

    #[test]
    fn solve_first_part_real() {
        let result = solve_first_part(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 6018);
    }

    #[test]
    fn solve_second_part_example() {
        let result = solve_second_part(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn solve_second_part_example_2() {
        let result = solve_second_part(EXAMPLE_INPUT_2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 36);
    }

    #[test]
    fn solve_second_part_real() {
        let result = solve_second_part(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2619);
    }
}
