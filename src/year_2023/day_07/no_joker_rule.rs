use std::collections::HashSet;

use itertools::Itertools;

use super::{get_hand_type, get_max_duplicate_count, HandRule, HandType};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct NoJokerRule {
    r#type: HandType,
    cards: Vec<u32>,
}

impl HandRule for NoJokerRule {
    fn from_cards(cards: &str) -> Self {
        let cards = NoJokerRule::parse_cards(cards);
        Self {
            r#type: get_hand_type(&cards),
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
}
