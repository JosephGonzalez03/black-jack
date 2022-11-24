use crate::games::black_jack::BlackJack;

pub mod games;

fn main() {
    BlackJack::new(4).play();
}
