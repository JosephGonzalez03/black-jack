use std::io;
use rand::Rng;

enum Suit {
    CLUB,
    DIAMOND,
    HEART,
    SPADE,
}

enum Card {
    TWO(Suit),
    THREE(Suit),
    FOUR(Suit),
    FIVE(Suit),
    SIX(Suit),
    SEVEN(Suit),
    EIGHT(Suit),
    NINE(Suit),
    TEN(Suit),
    KING(Suit),
    QUEEN(Suit),
    JACK(Suit),
    ACE(u16, Suit),
}

enum GameState {
    WIN,
    LOSE,
    CONTINUE,
}

struct BlackJack {
    card_deck: Vec<Card>,
    player_hand: Vec<Card>,
}

impl BlackJack {
    fn prepare_deck(&mut self) {
        self.card_deck.push(Card::TWO(Suit::CLUB));
        self.card_deck.push(Card::TWO(Suit::DIAMOND));
        self.card_deck.push(Card::TWO(Suit::HEART));
        self.card_deck.push(Card::TWO(Suit::SPADE));
        self.card_deck.push(Card::THREE(Suit::CLUB));
        self.card_deck.push(Card::THREE(Suit::DIAMOND));
        self.card_deck.push(Card::THREE(Suit::HEART));
        self.card_deck.push(Card::THREE(Suit::SPADE));
        self.card_deck.push(Card::FOUR(Suit::CLUB));
        self.card_deck.push(Card::FOUR(Suit::DIAMOND));
        self.card_deck.push(Card::FOUR(Suit::HEART));
        self.card_deck.push(Card::FOUR(Suit::SPADE));
        self.card_deck.push(Card::FIVE(Suit::CLUB));
        self.card_deck.push(Card::FIVE(Suit::DIAMOND));
        self.card_deck.push(Card::FIVE(Suit::HEART));
        self.card_deck.push(Card::FIVE(Suit::SPADE));
        self.card_deck.push(Card::SIX(Suit::CLUB));
        self.card_deck.push(Card::SIX(Suit::DIAMOND));
        self.card_deck.push(Card::SIX(Suit::HEART));
        self.card_deck.push(Card::SIX(Suit::SPADE));
        self.card_deck.push(Card::SEVEN(Suit::CLUB));
        self.card_deck.push(Card::SEVEN(Suit::DIAMOND));
        self.card_deck.push(Card::SEVEN(Suit::HEART));
        self.card_deck.push(Card::SEVEN(Suit::SPADE));
        self.card_deck.push(Card::EIGHT(Suit::CLUB));
        self.card_deck.push(Card::EIGHT(Suit::DIAMOND));
        self.card_deck.push(Card::EIGHT(Suit::HEART));
        self.card_deck.push(Card::EIGHT(Suit::SPADE));
        self.card_deck.push(Card::NINE(Suit::CLUB));
        self.card_deck.push(Card::NINE(Suit::DIAMOND));
        self.card_deck.push(Card::NINE(Suit::HEART));
        self.card_deck.push(Card::NINE(Suit::SPADE));
        self.card_deck.push(Card::TEN(Suit::CLUB));
        self.card_deck.push(Card::TEN(Suit::DIAMOND));
        self.card_deck.push(Card::TEN(Suit::HEART));
        self.card_deck.push(Card::TEN(Suit::SPADE));
        self.card_deck.push(Card::KING(Suit::CLUB));
        self.card_deck.push(Card::KING(Suit::DIAMOND));
        self.card_deck.push(Card::KING(Suit::HEART));
        self.card_deck.push(Card::KING(Suit::SPADE));
        self.card_deck.push(Card::QUEEN(Suit::CLUB));
        self.card_deck.push(Card::QUEEN(Suit::DIAMOND));
        self.card_deck.push(Card::QUEEN(Suit::HEART));
        self.card_deck.push(Card::QUEEN(Suit::SPADE));
        self.card_deck.push(Card::JACK(Suit::CLUB));
        self.card_deck.push(Card::JACK(Suit::DIAMOND));
        self.card_deck.push(Card::JACK(Suit::HEART));
        self.card_deck.push(Card::JACK(Suit::SPADE));
        self.card_deck.push(Card::ACE(1,Suit::CLUB));
        self.card_deck.push(Card::ACE(1,Suit::DIAMOND));
        self.card_deck.push(Card::ACE(1,Suit::HEART));
        self.card_deck.push(Card::ACE(1,Suit::SPADE));
    }
    fn draw_card(&mut self) -> Card {
        let mut rng = rand::thread_rng();
        let new_index = rng.gen_range(0..=self.card_deck.len());
        let new_card = self.card_deck.remove(new_index);
        return new_card
    }

    fn add_to_hand(&mut self, mut card: Card) {
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

    fn get_game_state(&self) -> GameState {
        let mut sum = 0;

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
                Card::ACE(value,_) => sum + value,
            }
        }
        println!("You're hand is at {}", sum);

        if sum > 21 {
            return GameState::LOSE;
        } else if sum == 21 {
            return GameState::WIN;
        }

        return GameState::CONTINUE;
    }
}

fn main() {
    //let mut dealer_hand: [i32; 52] = [0;52];
    let mut game = BlackJack {
        card_deck: Vec::new(),
        player_hand: Vec::new(),
    };

    game.prepare_deck();
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
