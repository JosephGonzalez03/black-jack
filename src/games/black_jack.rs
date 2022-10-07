use std::io;
use rand::Rng;
use super::GameState;
use super::Suit;
use super::Card;

struct BlackJack {
    card_deck: Vec<Card>,
    player_hand: Vec<Card>,
}

impl BlackJack {
    fn new() -> BlackJack {
        let mut card_deck = Vec::new();
        let player_hand = Vec::new();

        card_deck.push(Card::TWO(Suit::CLUB));
        card_deck.push(Card::TWO(Suit::DIAMOND));
        card_deck.push(Card::TWO(Suit::HEART));
        card_deck.push(Card::TWO(Suit::SPADE));
        card_deck.push(Card::THREE(Suit::CLUB));
        card_deck.push(Card::THREE(Suit::DIAMOND));
        card_deck.push(Card::THREE(Suit::HEART));
        card_deck.push(Card::THREE(Suit::SPADE));
        card_deck.push(Card::FOUR(Suit::CLUB));
        card_deck.push(Card::FOUR(Suit::DIAMOND));
        card_deck.push(Card::FOUR(Suit::HEART));
        card_deck.push(Card::FOUR(Suit::SPADE));
        card_deck.push(Card::FIVE(Suit::CLUB));
        card_deck.push(Card::FIVE(Suit::DIAMOND));
        card_deck.push(Card::FIVE(Suit::HEART));
        card_deck.push(Card::FIVE(Suit::SPADE));
        card_deck.push(Card::SIX(Suit::CLUB));
        card_deck.push(Card::SIX(Suit::DIAMOND));
        card_deck.push(Card::SIX(Suit::HEART));
        card_deck.push(Card::SIX(Suit::SPADE));
        card_deck.push(Card::SEVEN(Suit::CLUB));
        card_deck.push(Card::SEVEN(Suit::DIAMOND));
        card_deck.push(Card::SEVEN(Suit::HEART));
        card_deck.push(Card::SEVEN(Suit::SPADE));
        card_deck.push(Card::EIGHT(Suit::CLUB));
        card_deck.push(Card::EIGHT(Suit::DIAMOND));
        card_deck.push(Card::EIGHT(Suit::HEART));
        card_deck.push(Card::EIGHT(Suit::SPADE));
        card_deck.push(Card::NINE(Suit::CLUB));
        card_deck.push(Card::NINE(Suit::DIAMOND));
        card_deck.push(Card::NINE(Suit::HEART));
        card_deck.push(Card::NINE(Suit::SPADE));
        card_deck.push(Card::TEN(Suit::CLUB));
        card_deck.push(Card::TEN(Suit::DIAMOND));
        card_deck.push(Card::TEN(Suit::HEART));
        card_deck.push(Card::TEN(Suit::SPADE));
        card_deck.push(Card::KING(Suit::CLUB));
        card_deck.push(Card::KING(Suit::DIAMOND));
        card_deck.push(Card::KING(Suit::HEART));
        card_deck.push(Card::KING(Suit::SPADE));
        card_deck.push(Card::QUEEN(Suit::CLUB));
        card_deck.push(Card::QUEEN(Suit::DIAMOND));
        card_deck.push(Card::QUEEN(Suit::HEART));
        card_deck.push(Card::QUEEN(Suit::SPADE));
        card_deck.push(Card::JACK(Suit::CLUB));
        card_deck.push(Card::JACK(Suit::DIAMOND));
        card_deck.push(Card::JACK(Suit::HEART));
        card_deck.push(Card::JACK(Suit::SPADE));
        card_deck.push(Card::ACE(1, Suit::CLUB));
        card_deck.push(Card::ACE(1, Suit::DIAMOND));
        card_deck.push(Card::ACE(1, Suit::HEART));
        card_deck.push(Card::ACE(1, Suit::SPADE));

        BlackJack {
           card_deck,
           player_hand,
        }
    }

    fn draw_card(&mut self) -> Card {
        let mut rng = rand::thread_rng();
        let new_index = rng.gen_range(0..=self.card_deck.len());
        let new_card = self.card_deck.remove(new_index);
        return new_card;
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
                Card::ACE(value, _) => sum + value,
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
