use std::io;
use std::cmp::Ordering;
use super::{Card,CardDeck,Status,Player};

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

        for player in players.iter_mut() {
            'draw: loop {
                let mut answer = String::new();

                std::process::Command::new("clear").status().unwrap();
                show_cards(player);
                println!("Your hand is at {}", count_hand(player));

                println!("Would you like a card?");
                io::stdin()
                    .read_line(&mut answer)
                    .expect("Failed to read line.");

                if answer.trim().eq("y") {
                    player.add(draw(&mut deck));

                    let sum = count_hand(player);
                    if sum > 21 {
                        break 'draw;
                    }
                } else {
                    break 'draw;
                }
            }

        }

        // Determine who is closest to 21 and who is over 21
        for player in players.iter_mut() {
            let sum = count_hand(&player);
            if sum == 21 {
                player.set_status(Status::WIN)
            } else if sum > 21 {
                player.set_status(Status::LOSE);
            }
        }

        if players.iter().any(|player| player.get_status() == &Status::WIN) {
            players.retain(|player| player.get_status().eq(&Status::WIN));
        } else {
            players.retain(|player| player.get_status().eq(&Status::PLAYING));
            players.sort_by(|p1, p2| {
                let diff = count_hand(p2) - count_hand(p1);
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
        }
        println!("Winner is Player {}", players.get(0).unwrap().get_number());
    }
}

fn draw(deck: &mut CardDeck) -> Card {
    let mut card = deck.draw();
    if let Card::ACE(_, suit) = card {
        'ace_value: loop {
            let mut answer = String::new();
            println!("You drew an ace. Would you to treat it as a 1 or 11?");
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line.");

            let answer: i16 = answer.trim().parse().expect("Please type a number!");

            if answer == 1 || answer == 11 {
                card = Card::ACE(answer, suit);
                break 'ace_value;
            }
        }
    }
    card
}

fn show_cards(player: &Player) {
    println!("Player {}\n", player.get_number());
    for card in player.get_cards() {
        println!("{}", card);
    }
}

fn count_hand(player: &Player) -> i16 {
    let mut sum: i16 = 0;

    for card in player.get_cards() {
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
    }
    sum
}
