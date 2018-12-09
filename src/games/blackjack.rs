extern crate rand;

use std::fmt;

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

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {:?}", self.name, self.suit)
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

    pub fn hit(&mut self) -> String {
        self.player_cards.push(self.deck.cards.pop().unwrap());
        if score_cards(&self.player_cards) > 21 {
            self.state = GameState::Lost;
        }
        self.hand_in_words()
    }

    pub fn stay(&mut self) -> String {
        self.state = GameState::DealerTurn;
        self.dealer_play()
    }


    pub fn hand_in_words(&self) -> String {
        match self.state {
            GameState::PlayerTurn => format!{"Dealer: Face Down, {}\nPlayer: {}", self.dealer_cards[1], join_cards(&self.player_cards)},
            GameState::DealerTurn => format!{"Dealer: {}\nPlayer: {}", join_cards(&self.dealer_cards), join_cards(&self.player_cards)},
            GameState::Lost => format!("You busted! {}", join_cards(&self.player_cards)),
            GameState::Won => format!("You Won! {}", join_cards(&self.player_cards)),
        }
    }

    fn dealer_play(&mut self) -> String {
        if score_cards(&self.player_cards) > score_cards(&self.dealer_cards) {
            self.state = GameState::Won;
        } else {
            self.state = GameState::Lost;
        }
        self.hand_in_words()
    }
}

fn join_cards(cards: &Vec<Card>) -> String {
    cards.into_iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", ")
}

fn score_cards(cards: &Vec<Card>) -> u8 {
    let mut val_tot = 0;
    let mut alt_tot = 0;
    for card in cards {
        val_tot += card.value;
        alt_tot += card.alt_value;
    }

    if val_tot > 22 {
        return alt_tot;
    }

    val_tot 
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
