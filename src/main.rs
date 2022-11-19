use crate::games::black_jack::BlackJack;

pub mod games;

fn main() {
    //let mut dealer_hand: [i32; 52] = [0;52];
    let game = BlackJack::new(2);

    game.play();
}
