use std::io;
use rand::Rng;

fn main() {
    let mut card_deck: [i32; 52] = [2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5,6,6,6,6,7,7,7,7,8,8,8,8,9,9,9,9,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,-1,-1,-1,-1];
    let mut player_hand: [i32; 52] = [0;52];
    //let mut dealer_hand: [i32; 52] = [0;52];
    let mut turn = 0;

    let mut rng = rand::thread_rng();

    'game: loop {
        let mut answer = String::new();

        println!("Would you like a card?");
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line.");

        if answer.trim().eq("y") {
            let mut new_card;

            'turn: loop {
                let new_index = rng.gen_range(0..=51);
                new_card = card_deck[new_index];
                if new_card != 0 {
                    card_deck[new_index] = 0;
                    if new_card == -1 {
                        'ace_value: loop {
                            let mut answer = String::new();
                            println!("You drew an ace. Would you to treat it as a 1 or 11?");
                            io::stdin()
                                .read_line(&mut answer)
                                .expect("Failed to read line.");

                            let answer: i32 = answer.trim().parse().expect("Please type a number!");

                            if answer == 1 || answer == 11 {
                                new_card = answer;
                                break 'ace_value;
                            }
                        }
                    }
                    player_hand[turn] = new_card;
                    println!("{}", new_card);

                    let mut sum = 0;
                    for card in player_hand {
                        sum = sum + card;
                    }
                    println!("You're hand is at {}", sum);

                    if sum > 21 {
                        println!("You lose!");
                        break 'game;
                    } else if sum == 21 {
                        println!("You win!");
                        break 'game;
                    }
                    break 'turn;
                }
            }
        } else {
            println!("You are holding.");
        }
        turn = turn + 1;
    }
}
