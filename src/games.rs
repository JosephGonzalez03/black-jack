use strum_macros::{EnumIter};
pub mod black_jack;

pub enum GameState {
    WIN,
    LOSE,
    CONTINUE,
}

#[derive(EnumIter, Default, Copy, Clone, Debug, PartialEq)]
pub enum Suit {
    #[default]
    CLUB,
    DIAMOND,
    HEART,
    SPADE,
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
