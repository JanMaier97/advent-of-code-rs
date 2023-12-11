use std::collections::HashSet;

use itertools::Itertools;

use crate::{print_challenge_header, MyResult};

use self::{joker_rule::JokerRule, no_joker_rule::NoJokerRule};

mod joker_rule;
mod no_joker_rule;

const INPUT: &str = include_str!("input.txt");

#[derive(PartialEq, PartialOrd, Debug, Clone)]
enum HandType {
    Five = 7,
    Four = 6,
    FullHouse = 5,
    Triple = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn get_hand_type(cards: &[u32]) -> HandType {
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

fn get_max_duplicate_count(cards: &[u32]) -> usize {
    cards
        .into_iter()
        .unique()
        .map(|c| count_element(c, cards))
        .max()
        .unwrap()
}

fn count_element(element: &u32, cards: &[u32]) -> usize {
    cards.iter().filter(|&c| element == c).collect_vec().len()
}

#[derive(Debug)]
struct Bid<THand>
where
    THand: HandRule,
{
    hand: THand,
    bid: u32,
}

impl<THand: HandRule> Bid<THand> {
    fn from_line(line: &str) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();

        Self {
            bid: bid.parse::<u32>().unwrap(),
            hand: THand::from_cards(cards),
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
    let mut bids = parse_input::<NoJokerRule>(input);

    bids.sort_unstable_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());

    bids.iter()
        .enumerate()
        .map(|(index, bid)| ((index + 1) as u32) * bid.bid)
        .sum()
}

fn solve_part_two(input: &str) -> u32 {
    let mut bids = parse_input::<JokerRule>(input);

    bids.sort_unstable_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());

    bids.iter()
        .enumerate()
        .map(|(index, bid)| ((index + 1) as u32) * bid.bid)
        .sum()
}

fn parse_input<THand: HandRule>(input: &str) -> Vec<Bid<THand>> {
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

    #[test]
    fn part_two_real_input_correct() {
        let result = solve_part_two(INPUT);
        assert_eq!(result, 250757288);
    }
}

trait HandRule {
    fn from_cards(cards: &str) -> Self;
    fn r#type(&self) -> HandType;
    fn cards(&self) -> Vec<u32>;
}
