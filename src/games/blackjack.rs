extern crate rand;

use std::fmt;

use self::rand::seq::SliceRandom;
use self::rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    PlayerTurn,
    DealerTurn,
    Busted,
    Blackjack,
    Push,
    Won,
    Lost,
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub name: String,
    pub value: u8,
}

impl Card {
    pub fn new(suit: Suit, name: String, value: u8) -> Card {
        Card { suit, name, value }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self.suit {
            Suit::Spades => "\u{2660}",
            Suit::Hearts => "\u{2665}",
            Suit::Diamonds => "\u{2666}",
            Suit::Clubs => "\u{2663}",
        };

        write!(f, "{}{}", self.name, s)
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
                    deck.cards.push(Card::new(*suit, i.to_string(), i));
                }
                deck.cards.push(Card::new(*suit, String::from("J"), 10));
                deck.cards.push(Card::new(*suit, String::from("Q"), 10));
                deck.cards.push(Card::new(*suit, String::from("K"), 10));
                deck.cards.push(Card::new(*suit, String::from("A"), 11));
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
    pub player_hand: Vec<Card>,
    pub dealer_hand: Vec<Card>,
    pub state: GameState,
    pub deck: Deck,
}

impl Game {
    pub fn new(player_id: &str, wager: u64) -> Game {
        let mut game = Game {
            player_id: player_id.to_string(),
            wager,
            player_hand: Vec::new(),
            dealer_hand: Vec::new(),
            state: GameState::PlayerTurn,
            deck: Deck::new(2),
        };
        game.deck.shuffle();
        game.flop();
        game
    }

    fn flop(&mut self) -> String {
        self.player_hand.push(self.deck.deal_card().unwrap());
        self.dealer_hand.push(self.deck.deal_card().unwrap());
        self.player_hand.push(self.deck.deal_card().unwrap());
        self.dealer_hand.push(self.deck.deal_card().unwrap());

        let ps = score_hand(&self.player_hand);
        let ds = score_hand(&self.dealer_hand);

        // Check for Blackjack
        if ps == 21 && ds != 21 {
            // Blackjack
            self.state = GameState::Push;
        } else if ps == 21 {
            self.state = GameState::Blackjack;
        }

        self.hand_in_words()
    }

    pub fn hit(&mut self) -> String {
        if self.state == GameState::PlayerTurn {
            self.player_hand.push(self.deck.deal_card().unwrap());
            if score_hand(&self.player_hand) > 21 {
                self.state = GameState::Busted;
            }
        }
        self.hand_in_words()
    }

    pub fn stay(&mut self) -> String {
        if self.state == GameState::PlayerTurn {
            self.state = GameState::DealerTurn;
            return self.dealer_play();
        }
        self.hand_in_words()
    }

    fn dealer_play(&mut self) -> String {
        while score_hand(&self.dealer_hand) < 17 {
            self.dealer_hand.push(self.deck.deal_card().unwrap());
        }

        let ps = score_hand(&self.player_hand);
        let ds = score_hand(&self.dealer_hand);

        if ps > ds {
            self.state = GameState::Won;
        } else if ps == ds {
            self.state = GameState::Push;
        } else {
            self.state = GameState::Lost;
        }

        self.hand_in_words()
    }

    pub fn hand_in_words(&self) -> String {
        match self.state {
            GameState::PlayerTurn => format!(
                "Dealer: Face Down, {}\nPlayer: {}\n\nHit or Stay?",
                self.dealer_hand[1],
                join_cards(&self.player_hand)
            ),
            GameState::DealerTurn => format!(
                "Dealer: {}\nPlayer: {}",
                join_cards(&self.dealer_hand),
                join_cards(&self.player_hand)
            ),
            GameState::Blackjack => format!(
                "*You've got Blackjack!*\nDealer: {}\nPlayer: {}",
                join_cards(&self.dealer_hand),
                join_cards(&self.player_hand)
            ),

            GameState::Push => format!(
                "*It's a Push!*\nDealer: {}\nPlayer: {}",
                join_cards(&self.dealer_hand),
                join_cards(&self.player_hand)
            ),

            GameState::Busted => format!(
                "*You Busted!*\nDealer: {}\nPlayer: {}",
                join_cards(&self.dealer_hand),
                join_cards(&self.player_hand)
            ),

            GameState::Lost => format!(
                "*You Lost!*\nDealer: {}\nPlayer: {}",
                join_cards(&self.dealer_hand),
                join_cards(&self.player_hand)
            ),

            GameState::Won => format!(
                "*You Won!*\nDealer: {}\nPlayer: {}",
                join_cards(&self.dealer_hand),
                join_cards(&self.player_hand)
            ),
        }
    }
}

fn join_cards(cards: &[Card]) -> String {
    cards
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

fn score_hand(hand: &[Card]) -> u8 {
    let mut total: u8 = hand.iter().map(|x| x.value).sum();
    let num_aces = hand.iter().filter(|x| x.name == "A").count();
    for _ in 0..num_aces {
        if total > 21 {
            total -= 10;
        } else {
            break;
        }
    }

    total
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
        assert_eq!(g.player_hand.len(), 2);
        assert_eq!(g.dealer_hand.len(), 2);
    }

    #[test]
    fn test_score_hand_blackjack() {
        let mut test_hand: Vec<Card> = Vec::new();
        test_hand.push(Card::new(Suit::Spades, "A".to_string(), 11));
        test_hand.push(Card::new(Suit::Spades, "K".to_string(), 10));

        assert_eq!(score_hand(&test_hand), 21);
    }
    #[test]
    fn test_score_hand_bust() {
        let mut test_hand: Vec<Card> = Vec::new();
        test_hand.push(Card::new(Suit::Hearts, "K".to_string(), 10));
        test_hand.push(Card::new(Suit::Spades, "K".to_string(), 10));
        test_hand.push(Card::new(Suit::Spades, "Q".to_string(), 10));

        assert_eq!(score_hand(&test_hand), 30);
    }

    #[test]
    fn test_score_hand_ace_reduced() {
        let mut test_hand: Vec<Card> = Vec::new();
        test_hand.push(Card::new(Suit::Hearts, "K".to_string(), 10));
        test_hand.push(Card::new(Suit::Spades, "K".to_string(), 10));
        test_hand.push(Card::new(Suit::Spades, "A".to_string(), 11));

        assert_eq!(score_hand(&test_hand), 21);
    }

    #[test]
    fn test_score_hand_not_all_ace_reduced() {
        let mut test_hand: Vec<Card> = Vec::new();
        test_hand.push(Card::new(Suit::Hearts, "8".to_string(), 8));
        test_hand.push(Card::new(Suit::Spades, "A".to_string(), 11));
        test_hand.push(Card::new(Suit::Spades, "A".to_string(), 11));

        assert_eq!(score_hand(&test_hand), 20);
    }
}
