use strum::IntoEnumIterator;
use rand::Rng;
use std::io;
use std::cmp::Ordering;
use super::{Card,Suit,Player};

pub struct BlackJack {
    players: usize,
}

impl BlackJack {
    pub fn new(players: usize) -> Self {
        Self {
            players,
        }
    }

    pub fn play(&self) {
        let mut deck = CardDeck::new();
        let mut players: Vec<Player> = Vec::new();

        for player in 0..self.players {
            players.push(Player::new(player+1));
        }

        players.iter_mut().for_each(|player| {
            'draw: loop {
                let mut answer = String::new();

                std::process::Command::new("clear").status().unwrap();
                player.show_cards();
                println!("Would you like a card?");
                io::stdin()
                    .read_line(&mut answer)
                    .expect("Failed to read line.");

                if answer.trim().eq("y") {
                    player.add(deck.draw());

                    if player.count_hand() > 21 {
                        break 'draw;
                    }
                } else {
                    break 'draw;
                }
            }
        });

        std::process::Command::new("clear").status().unwrap();
        players.iter().for_each(|player| player.show_cards());
        players.sort_by(|p1, p2| {
            let diff = p2.count_hand() as i16 - p1.count_hand() as i16;
            let order;

            if diff > 0 {
                order = Ordering::Greater;
            } else if diff == 0 {
                order = Ordering::Equal;
            } else {
                order = Ordering::Less;
            }
            order
        });
        players.retain(|player| player.count_hand() <= 21);
        println!("++++++++++++++++++++++");
        println!("+ Winner is Player {} +", players.get(0).unwrap().get_number());
        println!("++++++++++++++++++++++");
    }
}

pub struct CardDeck {
    cards: Vec<Card>,
}

impl CardDeck {
    fn new() -> Self {
        let mut cards = Vec::new();

        for suit in Suit::iter() {
            cards.push(Card::TWO(2, suit));
            cards.push(Card::THREE(3, suit));
            cards.push(Card::FOUR(4, suit));
            cards.push(Card::FIVE(5, suit));
            cards.push(Card::SIX(6, suit));
            cards.push(Card::SEVEN(7, suit));
            cards.push(Card::EIGHT(8, suit));
            cards.push(Card::NINE(9, suit));
            cards.push(Card::TEN(10, suit));
            cards.push(Card::KING(10, suit));
            cards.push(Card::QUEEN(10, suit));
            cards.push(Card::JACK(10, suit));
            cards.push(Card::ACE(1, suit));
        }

        Self {
            cards,
        }
    }

    fn draw(&mut self) -> Card {
        let mut rng = rand::thread_rng();
        let new_index = rng.gen_range(0..self.cards.len());
        let mut card = self.cards.remove(new_index);

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
        card
    }
}
