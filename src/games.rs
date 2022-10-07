pub mod black_jack;

enum GameState {
    WIN,
    LOSE,
    CONTINUE,
}

enum Suit {
    CLUB,
    DIAMOND,
    HEART,
    SPADE,
}

enum Card {
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
