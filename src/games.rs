use rand::Rng;
use strum::IntoEnumIterator;
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

pub struct CardDeck {
    cards: Vec<Card>,
}

impl CardDeck {
    fn new() -> CardDeck {
        let mut cards = Vec::new();

        for suit in Suit::iter() {
            cards.push(Card::TWO(suit));
            cards.push(Card::THREE(suit));
            cards.push(Card::FOUR(suit));
            cards.push(Card::FIVE(suit));
            cards.push(Card::SIX(suit));
            cards.push(Card::SEVEN(suit));
            cards.push(Card::EIGHT(suit));
            cards.push(Card::NINE(suit));
            cards.push(Card::TEN(suit));
            cards.push(Card::KING(suit));
            cards.push(Card::QUEEN(suit));
            cards.push(Card::JACK(suit));
            cards.push(Card::ACE(1, suit));
        }

        CardDeck {
            cards,
        }
    }

    fn draw(&mut self) -> Card {
        let mut rng = rand::thread_rng();
        let new_index = rng.gen_range(0..=self.cards.len());
        let new_card = self.cards.remove(new_index);
        return new_card;
    }

}

pub struct Player {
    cards: Vec<Card>,
}

impl Player {
    fn new() -> Player {
        Player {
            cards: Vec::<Card>::new(),
        }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }
}
