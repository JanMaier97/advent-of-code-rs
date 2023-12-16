use std::collections::HashMap;

use itertools::Itertools;

use super::{count_element, get_hand_type, HandRule, HandType};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct JokerRule {
    r#type: HandType,
    cards: Vec<u32>,
}

impl JokerRule {
    fn parse_cards(card_chars: &str) -> Vec<u32> {
        card_chars
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                c => c.to_digit(10).unwrap(),
            })
            .collect_vec()
    }

    fn get_optimal_cards(cards: &[u32]) -> Vec<u32> {
        let non_joker_cards = cards
            .iter()
            .copied()
            .filter(|&c| c != 1)
            .collect::<Vec<_>>();

        let counts_by_number = non_joker_cards
            .iter()
            .map(|c| (c, count_element(c, &non_joker_cards)))
            .collect::<HashMap<&u32, usize>>();

        let best_card = counts_by_number
            .iter()
            .max_by(|x, y| x.1.cmp(y.1))
            .map(|(&number, _)| *number)
            .unwrap_or(2);

        cards
            .iter()
            .map(|&c| if c == 1 { best_card } else { c })
            .collect_vec()
    }
}

impl HandRule for JokerRule {
    fn from_cards(card_chars: &str) -> Self {
        let original_cards = JokerRule::parse_cards(card_chars);
        let best_cards = JokerRule::get_optimal_cards(&original_cards);

        JokerRule {
            r#type: get_hand_type(&best_cards),
            cards: original_cards,
        }
    }

    fn r#type(&self) -> HandType {
        self.r#type.clone()
    }

    fn cards(&self) -> Vec<u32> {
        self.cards.clone()
    }
}
