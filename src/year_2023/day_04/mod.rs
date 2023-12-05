use std::collections::HashMap;

use crate::{print_challenge_header, MyResult};

const INPUT: &str = include_str!("input.txt");

struct Cards {
    id: usize,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Cards {
    fn points(&self) -> u32 {
        let numbers = self.get_matching_numbers();

        if numbers.len() == 0 {
            return 0;
        }

        numbers.iter().skip(1).fold(1, |sum, _| sum * 2)
    }

    fn get_matching_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .copied()
            .filter(|n| self.winning_numbers.contains(n))
            .collect::<Vec<_>>()
    }
}

pub fn solve() -> MyResult<()> {
    print_challenge_header(4);

    println!("{}", solve_part_one(INPUT));
    println!("{}", solve_part_two(INPUT));
    Ok(())
}

fn solve_part_one(input: &str) -> u32 {
    let cards = parse_input(input);

    let total_score = cards.iter().map(|c| c.points()).sum();

    total_score
}

fn solve_part_two(input: &str) -> u32 {
    let cards = parse_input(input);
    let mut card_count_mapping = cards
        .iter()
        .map(|c| (c.id, 1_u32))
        .collect::<HashMap<_, _>>();

    for card in cards {
        let ids_to_duplicate = card
            .get_matching_numbers()
            .into_iter()
            .enumerate()
            .map(|(index, _)| card.id + index + 1);

        for id in ids_to_duplicate {
            let count = *card_count_mapping.get(&id).unwrap();
            let current_card_count = *card_count_mapping.get(&card.id).unwrap();
            card_count_mapping.insert(id, count + current_card_count);
        }
    }

    card_count_mapping.into_iter().map(|(_, count)| count).sum()
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
    use crate::year_2023::day_04::{solve_part_two, INPUT};

    use super::solve_part_one;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn example_input_part_one_solved_correctly() {
        let result = solve_part_one(EXAMPLE_INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn real_input_part_one_solved_correctly() {
        let result = solve_part_one(INPUT);
        assert_eq!(result, 20829);
    }

    #[test]
    fn example_input_part_two_solved_correctly() {
        let result = solve_part_two(EXAMPLE_INPUT);
        assert_eq!(result, 30);
    }

    #[test]
    fn real_input_part_two_solved_correctly() {
        let result = solve_part_two(INPUT);
        assert_eq!(result, 12648035);
    }
}
