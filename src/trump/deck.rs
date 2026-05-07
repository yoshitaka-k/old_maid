/// トランプカードの山札

use super::card::Card;
use rand::seq::SliceRandom;
use std::collections::VecDeque;

use crate::GameMode;
use crate::Field;

//////////////////////////////////////////////////

pub struct Deck {
    cards: VecDeque<Card>,
}

impl Deck {
    pub fn new(mode: &GameMode, field: &mut Field) -> Self {
        let mut cards = VecDeque::new();

        for suit in ["h", "d", "c", "s"] {
            for rank in ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13"] {
                cards.push_back(Card::new(String::from(suit), String::from(rank)));
            }
        }

        match mode {
            GameMode::OldMaid => {
                cards.push_back(Card::new(String::from("j"), String::from("0")));
                cards.make_contiguous().shuffle(&mut rand::thread_rng());
            },
            GameMode::OldMan => {
                cards.make_contiguous().shuffle(&mut rand::thread_rng());
                let joker_card = cards.pop_front().expect("REASON");
                field.set_joker(joker_card.get_name());
            }
        }

        Self { cards: cards }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }
}
