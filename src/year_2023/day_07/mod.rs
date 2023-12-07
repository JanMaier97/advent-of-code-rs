use std::collections::HashSet;

use itertools::Itertools;

use crate::{print_challenge_header, MyResult};

const INPUT: &str = include_str!("input.txt");

#[derive(PartialEq, PartialOrd, Debug)]
enum HandType {
    Five = 7,
    Four = 6,
    FullHouse = 5,
    Triple = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(PartialEq, PartialOrd, Debug)]
struct Hand {
    r#type: HandType,
    cards: Vec<u32>,
}

impl Hand {
    fn from_cards(cards: &str) -> Self {
        let cards = Hand::parse_cards(cards);
        Self {
            r#type: Hand::get_type(&cards),
            cards: cards,
        }
    }

    fn parse_cards(card_chars: &str) -> Vec<u32> {
        card_chars
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                c => c.to_digit(10).unwrap(),
            })
            .collect_vec()
    }

    fn get_type(cards: &[u32]) -> HandType {
        let unique_values = cards.iter().collect::<HashSet<_>>().len();

        let max_duplicate_count = get_max_duplicate_count(cards);

        match unique_values {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 if max_duplicate_count == 2 => HandType::TwoPair,
            3 => HandType::Triple,
            2 if max_duplicate_count == 4 => HandType::Four,
            2 => HandType::FullHouse,
            1 => HandType::Five,
            _ => panic!(),
        }
    }
}

fn get_max_duplicate_count(cards: &[u32]) -> usize {
    cards
        .into_iter()
        .unique()
        .map(|c| cards.iter().filter(|&c2| c2 == c).collect_vec().len())
        .max()
        .unwrap()
}

#[derive(Debug)]
struct Bid {
    hand: Hand,
    bid: u32,
}

impl Bid {
    fn from_line(line: &str) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();

        Self {
            bid: bid.parse::<u32>().unwrap(),
            hand: Hand::from_cards(cards),
        }
    }
}

pub fn solve() -> MyResult<()> {
    print_challenge_header(7);

    println!("The total winnings are {}", solve_part_one(INPUT));
    println!("The total winnings are {}", solve_part_two(INPUT));

    Ok(())
}

fn solve_part_one(input: &str) -> u32 {
    let mut bids = parse_input(input);

    bids.sort_unstable_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());

    bids.iter()
        .enumerate()
        .map(|(index, bid)| ((index + 1) as u32) * bid.bid)
        .sum()
}

fn solve_part_two(input: &str) -> u32 {
    !unimplemented!()
}

fn parse_input(input: &str) -> Vec<Bid> {
    input.lines().map(|l| Bid::from_line(l)).collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::year_2023::day_07::{solve_part_two, INPUT};

    use super::solve_part_one;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn part_one_example_input_correct() {
        let result = solve_part_one(EXAMPLE_INPUT);
        assert_eq!(result, 6440);
    }

    #[test]
    fn part_one_real_input_correct() {
        let result = solve_part_one(INPUT);
        assert_eq!(result, 251287184);
    }

    #[test]
    fn part_two_example_input_correct() {
        let result = solve_part_two(EXAMPLE_INPUT);
        assert_eq!(result, 5905);
    }
}
