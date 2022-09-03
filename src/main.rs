use std::io;
use rand::Rng;

enum GameState {
    WIN,
    LOSE,
    CONTINUE,
}

struct Game {
    card_deck: [i32; 52],
    player_hand: [i32; 52],
    turn: usize,
}

impl Game {
    fn draw_card(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let mut new_card;
        'draw: loop {
            let new_index = rng.gen_range(0..=51);
            new_card = self.card_deck[new_index];
            if new_card != 0 {
                self.card_deck[new_index] = 0;
                break 'draw;
            }
        }
        return new_card
    }

    fn add_to_hand(&mut self, mut card: i32) {
        if card == -1 {
            'ace_value: loop {
                let mut answer = String::new();
                println!("You drew an ace. Would you to treat it as a 1 or 11?");
                io::stdin()
                    .read_line(&mut answer)
                    .expect("Failed to read line.");

                let answer: i32 = answer.trim().parse().expect("Please type a number!");

                if answer == 1 || answer == 11 {
                    card = answer;
                    break 'ace_value;
                }
            }
        }
        self.player_hand[self.turn] = card;
    }

    fn get_game_state(&self) -> GameState {
        let mut sum = 0;
        for card in self.player_hand {
            sum = sum + card;
        }
        println!("You're hand is at {}", sum);

        if sum > 21 {
            return GameState::LOSE;
        } else if sum == 21 {
            return GameState::WIN;
        }

        return GameState::CONTINUE;

    }

    fn increase_turn(&mut self) {
        self.turn = self.turn + 1;
    }
}

fn main() {
    //let mut dealer_hand: [i32; 52] = [0;52];
    let mut game = Game {
        card_deck: [2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5,6,6,6,6,7,7,7,7,8,8,8,8,9,9,9,9,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,-1,-1,-1,-1],
        player_hand: [0;52],
        turn: 0,
    };

    'game: loop {
        let mut answer = String::new();

        println!("Would you like a card?");
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line.");

        if answer.trim().eq("y") {
            let card = game.draw_card();
            game.add_to_hand(card);
            match game.get_game_state() {
                GameState::CONTINUE => game.increase_turn(),
                GameState::WIN => {
                    println!("You win!");
                    break 'game;
                }
                GameState::LOSE => {
                    println!("You lose!");
                    break 'game;
                }
            }
        } else {
            println!("You are holding.");
        }
    }
}
