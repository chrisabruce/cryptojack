extern crate rand;

use self::rand::seq::SliceRandom;
use self::rand::thread_rng;

#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub name: String,
    pub value: u8,
    pub alt_value: u8,
}

impl Card {
    pub fn new(suit: Suit, name: String, value: u8, alt_value: u8) -> Card {
        Card {
            suit,
            name,
            value,
            alt_value,
        }
    }
}

impl ToString for Card {
    fn to_string(&self) -> String {
        format!("{} of {:?}", self.name, self.suit)
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(size: u8) -> Deck {
        let mut deck = Deck { cards: Vec::new() };

        let suits = vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];

        for _x in 0..size {
            for suit in &suits {
                for i in 2..10 {
                    deck.cards
                        .push(Card::new(suit.clone(), i.to_string(), i, i));
                }
                deck.cards
                    .push(Card::new(suit.clone(), String::from("Jack"), 10, 10));
                deck.cards
                    .push(Card::new(suit.clone(), String::from("Queen"), 10, 10));
                deck.cards
                    .push(Card::new(suit.clone(), String::from("King"), 10, 10));
                deck.cards
                    .push(Card::new(suit.clone(), String::from("Ace"), 11, 2));
            }
        }
        deck
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_new_deck_size() {
        let mut d = Deck::new(2);
        d.shuffle();
        assert_eq!(d.cards.len(), 96);

        let mut d = Deck::new(1);
        d.shuffle();
        assert_eq!(d.cards.len(), 48);
    }

}
