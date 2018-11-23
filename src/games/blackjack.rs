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

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    PlayerTurn,
    DealerTurn,
    Won,
    Lost,
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
                for i in 2..11 {
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

pub struct Game {
    pub player_id: String, 
    pub wager: u64,
    pub player_cards: Vec<Card>,
    pub dealer_cards: Vec<Card>,
    pub state: GameState,
    pub deck: Deck,
}

impl Game {
    pub fn new(player_id: &String, wager: u64) -> Game {
        let mut game = Game {
            player_id: player_id.clone(),
            wager: wager,
            player_cards: Vec::new(),
            dealer_cards: Vec::new(),
            state: GameState::PlayerTurn,
            deck: Deck::new(2),
        };
        game.deck.shuffle();
        game.flop();
        game
    }

    pub fn flop(&mut self) {
        self.player_cards.push(self.deck.deal_card().unwrap());
        self.dealer_cards.push(self.deck.deal_card().unwrap());
        self.player_cards.push(self.deck.deal_card().unwrap());
        self.dealer_cards.push(self.deck.deal_card().unwrap());
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_new_deck_size() {
        let mut d = Deck::new(2);
        d.shuffle();
        assert_eq!(d.cards.len(), 104);

        let mut d = Deck::new(1);
        d.shuffle();
        assert_eq!(d.cards.len(), 52);
    }

    #[test]
    fn test_new_game() {
        let g = Game::new(&String::from("test"), 500);
        assert_eq!(g.player_cards.len(), 2);
        assert_eq!(g.dealer_cards.len(), 2);
    }

}
