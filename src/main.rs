use std::io;
use crate::games::black_jack::BlackJack;
use crate::games::*;

pub mod games;

fn main() {
    //let mut dealer_hand: [i32; 52] = [0;52];
    let mut game = BlackJack::new();

    std::process::Command::new("clear").status().unwrap();
    'game: loop {
        let mut answer = String::new();

        println!("Would you like a card?");
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line.");

        if answer.trim().eq("y") {
            game.draw();
            match game.get_game_state() {
                GameState::CONTINUE => (),
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
