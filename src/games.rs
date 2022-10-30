use strum_macros::{EnumIter};
pub mod black_jack;

pub enum GameState {
    WIN,
    LOSE,
    CONTINUE,
}

#[derive(EnumIter, Copy, Clone, Debug)]
pub enum Suit {
    CLUBS,
    DIAMONDS,
    HEARTS,
    SPADES,
}

pub enum Card {
    TWO(Suit),
    THREE(Suit),
    FOUR(Suit),
    FIVE(Suit),
    SIX(Suit),
    SEVEN(Suit),
    EIGHT(Suit),
    NINE(Suit),
    TEN(Suit),
    KING(Suit),
    QUEEN(Suit),
    JACK(Suit),
    ACE(u16, Suit),
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Card::TWO(suit) => write!(f, "{} of {:?}", "Two", suit),
            Card::THREE(suit) => write!(f, "{} of {:?}", "Three", suit),
            Card::FOUR(suit) => write!(f, "{} of {:?}", "Four", suit),
            Card::FIVE(suit) => write!(f, "{} of {:?}", "Five", suit),
            Card::SIX(suit) => write!(f, "{} of {:?}", "Six", suit),
            Card::SEVEN(suit) => write!(f, "{} of {:?}", "Seven", suit),
            Card::EIGHT(suit) => write!(f, "{} of {:?}", "Eight", suit),
            Card::NINE(suit) => write!(f, "{} of {:?}", "Nine", suit),
            Card::TEN(suit) => write!(f, "{} of {:?}", "Ten", suit),
            Card::KING(suit) => write!(f, "{} of {:?}", "King", suit),
            Card::QUEEN(suit) => write!(f, "{} of {:?}", "Queen", suit),
            Card::JACK(suit) => write!(f, "{} of {:?}", "Jack", suit),
            Card::ACE(value, suit) => write!(f, "{} of {:?} ({})", "Ace", suit, value),
        }
    }
}
