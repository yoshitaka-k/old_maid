use crate::rand_range;
use crate::{GameMode, Card};

//////////////////////////////////////////////////

/// トランプカードの山札
pub struct Deck {
    cards: Vec<Card>,
    mystery_card: Vec<Card>,
}

impl Deck {
    pub fn new(mode: &GameMode) -> Self {
        let mut cards = Vec::new();
        let mut mystery_card = Vec::new();

        for suit in ["h", "d", "c", "s"] {
            for rank in ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13"] {
                cards.push(Card::new(String::from(suit), String::from(rank)));
            }
        }

        match mode {
            GameMode::OldMaid => {
                cards.push(Card::new(String::from("j"), String::from("0")));
            },
            GameMode::OldMan => {
                let index = rand_range(0..cards.len());
                mystery_card.push(cards.remove(index));
            },
        }

        Self {
            cards: cards,
            mystery_card: mystery_card,
        }
    }

    pub fn get_cards(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn has_mystery_card(&self) -> bool {
        self.mystery_card.is_empty() == false
    }

    pub fn pop_mystery_card(&mut self) -> Option<Card> {
        self.mystery_card.pop()
    }
}
