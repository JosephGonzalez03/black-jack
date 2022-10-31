use std::io;
use super::{Card,CardDeck,GameState};

pub struct BlackJack {
    card_deck: CardDeck,
    player_hand: Vec<Card>,
}

impl BlackJack {
    pub fn new() -> BlackJack {
        let card_deck = CardDeck::new();
        let player_hand = Vec::new();

        BlackJack {
           card_deck,
           player_hand,
        }
    }

    pub fn add_to_hand(&mut self, mut card: Card) {
        if let Card::ACE(_, suit) = card {
            'ace_value: loop {
                let mut answer = String::new();
                println!("You drew an ace. Would you to treat it as a 1 or 11?");
                io::stdin()
                    .read_line(&mut answer)
                    .expect("Failed to read line.");

                let answer: u16 = answer.trim().parse().expect("Please type a number!");

                if answer == 1 || answer == 11 {
                    card = Card::ACE(answer, suit);
                    break 'ace_value;
                }
            }
        }
        self.player_hand.push(card);
    }

    pub fn get_card_deck(&mut self) -> &mut CardDeck {
        &mut self.card_deck
    }

    pub fn get_game_state(&self) -> GameState {
        let mut sum = 0;

        std::process::Command::new("clear").status().unwrap();

        for card in &self.player_hand {
            sum = match card {
                Card::TWO(_) => sum + 2,
                Card::THREE(_) => sum + 3,
                Card::FOUR(_) => sum + 4,
                Card::FIVE(_) => sum + 5,
                Card::SIX(_) => sum + 6,
                Card::SEVEN(_) => sum + 7,
                Card::EIGHT(_) => sum + 8,
                Card::NINE(_) => sum + 9,
                Card::TEN(_) => sum + 10,
                Card::KING(_) => sum + 10,
                Card::QUEEN(_) => sum + 10,
                Card::JACK(_) => sum + 10,
                Card::ACE(value, _) => sum + value,
            };
            println!("{}", card);
        }
        println!("\nYou're hand is at {}", sum);

        if sum > 21 {
            return GameState::LOSE;
        } else if sum == 21 {
            return GameState::WIN;
        }

        return GameState::CONTINUE;
    }
}
