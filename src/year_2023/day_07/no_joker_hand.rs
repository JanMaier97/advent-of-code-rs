use std::collections::HashSet;

use itertools::Itertools;

use super::{HandType, get_max_duplicate_count, HandRule};


#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct NoJokerRule {
    r#type: HandType,
    cards: Vec<u32>,
}

impl HandRule for NoJokerRule {
    fn from_cards(cards: &str)-> Self {
        let cards = NoJokerRule::parse_cards(cards);
        Self {
            r#type: NoJokerRule::get_type(&cards),
            cards: cards,
        }
    }

    fn r#type(&self) -> HandType {
        self.r#type.clone()
    }

    fn cards(&self) -> Vec<u32> {
        self.cards.clone()
    }
}

impl NoJokerRule {
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