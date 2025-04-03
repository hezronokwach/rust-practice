use rand::prelude::*;
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Suit {
    pub fn random() -> Suit {
        let num = rand::rng().random_range(1..=4);
        Suit::translate(num)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 | _ => Suit::Club,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let num = rand::rng().random_range(1..=13);
        Rank::translate(num)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 | _ => Rank::King,
        }
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    matches!((&card.suit, &card.rank), (&Suit::Spade, &Rank::Ace))
}
