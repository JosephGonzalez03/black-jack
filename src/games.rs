use strum_macros::{EnumIter};

pub mod black_jack;

#[derive(PartialEq)]
pub enum Status {
    WIN,
    LOSE,
    PLAYING,
}

#[derive(EnumIter, Copy, Clone, Debug)]
pub enum Suit {
    CLUBS,
    DIAMONDS,
    HEARTS,
    SPADES,
}

pub enum Card {
    TWO(u16, Suit),
    THREE(u16, Suit),
    FOUR(u16, Suit),
    FIVE(u16, Suit),
    SIX(u16, Suit),
    SEVEN(u16, Suit),
    EIGHT(u16, Suit),
    NINE(u16, Suit),
    TEN(u16, Suit),
    KING(u16, Suit),
    QUEEN(u16, Suit),
    JACK(u16, Suit),
    ACE(u16, Suit),
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Card::TWO(value, suit) => write!(f, "{} of {:?} ({})", "Two", suit, value),
            Card::THREE(value, suit) => write!(f, "{} of {:?} ({})", "Three", suit, value),
            Card::FOUR(value, suit) => write!(f, "{} of {:?} ({})", "Four", suit, value),
            Card::FIVE(value, suit) => write!(f, "{} of {:?} ({})", "Five", suit, value),
            Card::SIX(value, suit) => write!(f, "{} of {:?} ({})", "Six", suit, value),
            Card::SEVEN(value, suit) => write!(f, "{} of {:?} ({})", "Seven", suit, value),
            Card::EIGHT(value, suit) => write!(f, "{} of {:?} ({})", "Eight", suit, value),
            Card::NINE(value, suit) => write!(f, "{} of {:?} ({})", "Nine", suit, value),
            Card::TEN(value, suit) => write!(f, "{} of {:?} ({})", "Ten", suit, value),
            Card::KING(value, suit) => write!(f, "{} of {:?} ({})", "King", suit, value),
            Card::QUEEN(value, suit) => write!(f, "{} of {:?} ({})", "Queen", suit, value),
            Card::JACK(value, suit) => write!(f, "{} of {:?} ({})", "Jack", suit, value),
            Card::ACE(value, suit) => write!(f, "{} of {:?} ({})", "Ace", suit, value),
        }
    }
}

pub struct Player {
    number: usize,
    cards: Vec<Card>,
    status: Status,
}

impl Player {
    fn new(number: usize) -> Self {
        Self {
            number,
            cards: Vec::<Card>::new(),
            status: Status::PLAYING,
        }
    }

    fn get_number(&self) -> usize {
        self.number
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }

    fn is_status(&self, status: Status) -> bool {
        self.status.eq(&status)
    }

    fn set_status(&mut self, new_status: Status) {
        self.status = new_status;
    }

    fn show_cards(&self) {
        println!("Player {}\n", self.get_number());
        self.cards.iter().for_each(|card| println!("{}", card));
    }

    fn count_hand(&self) -> u16 {
        let mut sum = 0;

        self.cards.iter().for_each(|card| {
            sum = match card {
                Card::TWO(value, _) => sum + value,
                Card::THREE(value, _) => sum + value,
                Card::FOUR(value, _) => sum + value,
                Card::FIVE(value, _) => sum + value,
                Card::SIX(value, _) => sum + value,
                Card::SEVEN(value, _) => sum + value,
                Card::EIGHT(value, _) => sum + value,
                Card::NINE(value, _) => sum + value,
                Card::TEN(value, _) => sum + value,
                Card::KING(value, _) => sum + value,
                Card::QUEEN(value, _) => sum + value,
                Card::JACK(value, _) => sum + value,
                Card::ACE(value, _) => sum + value,
            };
        });
        sum
    }
}
