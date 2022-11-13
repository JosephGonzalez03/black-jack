use std::io;
use super::{Card,CardDeck,GameState,Player};

pub struct BlackJack {
    players: usize,
}

impl BlackJack {
    pub fn new(players: usize) -> Self {

        Self {
            players,
        }
    }

    pub fn draw(deck: &mut CardDeck) -> Card {
        let mut card = deck.draw();
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

    pub fn get_game_state(&self, player: &Player) -> GameState {
        let mut sum = 0;

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
                println!("Player {}\n", player.get_number());

                match self.get_game_state(&player) {
                    GameState::CONTINUE => (),
                    GameState::WIN => {
                        println!("You win!");
                        break 'draw;
                    }
                    GameState::LOSE => {
                        println!("You lose!");
                        break 'draw;
                    }
                }

                println!("Would you like a card?");
                io::stdin()
                    .read_line(&mut answer)
                    .expect("Failed to read line.");

                if answer.trim().eq("y") {
                    player.add(BlackJack::draw(&mut deck));
                } else {
                    break 'draw;
                }
            }
        }
    }
}
