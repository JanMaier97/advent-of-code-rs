use itertools::Itertools;

use crate::{print_challenge_header, MyResult};

const INPUT: &str = include_str!("input.txt");

struct Cards {
    id: usize,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Cards {
    fn points(&self) -> u32 {
        let numbers = self
            .numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .collect_vec();

        if numbers.len() == 0 {
            return 0;
        }

        numbers.iter().skip(1).fold(1, |sum, _| sum * 2)
    }
}

pub fn solve() -> MyResult<()> {
    print_challenge_header(4);

    println!("{}", solve_part_one(INPUT));
    println!("{}", solve_part_one(INPUT));
    Ok(())
}

fn solve_part_one(input: &str) -> u32 {
    let cards = parse_input(input);

    let total_score = cards.iter().map(|c| c.points()).sum();

    total_score
}

fn solve_part_two(input: &str) -> u32 {
    unimplemented!()
}

fn parse_input(input: &str) -> Vec<Cards> {
    let mut games = Vec::new();

    for (line_index, line) in input.lines().enumerate() {
        let card_id = line_index + 1;

        let (_, card_numbers) = line
            .split_once(":")
            .expect(format!("Input on line {} is missing a ':'", card_id).as_str());

        let (winning_numbers, numbers) = card_numbers
            .split_once("|")
            .expect(format!("Input on line {} is missing a '|'", card_id).as_str());

        let winning_numbers = parse_number_list(winning_numbers);
        let numbers = parse_number_list(numbers);

        let game = Cards {
            id: card_id,
            winning_numbers,
            numbers,
        };
        games.push(game);
    }

    games
}

fn parse_number_list(number: &str) -> Vec<u32> {
    number
        .split(' ')
        .filter(|str| !str.is_empty())
        .map(|str| str.trim().parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .expect(format!("Failed to parse numbers: {}", number).as_str())
}

#[cfg(test)]
mod tests {
    use crate::year_2023::day_04::INPUT;

    use super::solve_part_one;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn example_input_solved_correctly() {
        let result = solve_part_one(EXAMPLE_INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn real_input_solved_correctly() {
        let result = solve_part_one(INPUT);
        assert_eq!(result, 20829);
    }
}
