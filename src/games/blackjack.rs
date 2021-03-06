use std::fmt;

use rand::seq::SliceRandom;
use rand::thread_rng;

use separator::Separatable;

const BLACKJACK_PAYOUT: f32 = 1.5;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    PlaceBet,
    PlayerTurn,
    DealerTurn,
    Busted,
    Blackjack,
    Push,
    Won,
    Lost,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Game {
    pub player_id: String,
    pub wager: u64,
    pub player_hand: Vec<Card>,
    pub dealer_hand: Vec<Card>,
    pub state: GameState,
    pub deck: Deck,
    pub payout: u64,
}

impl Game {
    pub fn new(player_id: &str) -> Game {
        let mut game = Game {
            player_id: player_id.to_string(),
            wager: 0,
            player_hand: Vec::new(),
            dealer_hand: Vec::new(),
            state: GameState::PlaceBet,
            deck: Deck::new(2),
            payout: 0,
        };
        game.deck.shuffle();
        game
    }

    pub fn bet(&mut self, wager: u64) -> String {
        if self.state == GameState::PlaceBet {
            self.wager = wager;
            self.state = GameState::PlayerTurn;
            self.flop();
        }
        self.hand_in_words()
    }

    fn flop(&mut self) {
        self.player_hand.push(self.deck.deal_card().unwrap());
        self.dealer_hand.push(self.deck.deal_card().unwrap());
        self.player_hand.push(self.deck.deal_card().unwrap());
        self.dealer_hand.push(self.deck.deal_card().unwrap());

        let ps = score_hand(&self.player_hand);
        let ds = score_hand(&self.dealer_hand);

        // Check for Blackjack
        if ps == 21 && ds == 21 {
            // Blackjack
            self.state = GameState::Push;
            self.payout = self.wager;
        } else if ps == 21 {
            self.state = GameState::Blackjack;
            self.payout = self.wager + (self.wager as f32 * BLACKJACK_PAYOUT) as u64;
        }
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
            self.payout = self.wager * 2;
        } else if ps == ds {
            self.state = GameState::Push;
            self.payout = self.wager;
        } else {
            self.state = GameState::Lost;
            self.payout = 0;
        }

        self.hand_in_words()
    }

    pub fn is_over(&self) -> bool {
        self.state != GameState::PlaceBet
            && self.state != GameState::PlayerTurn
            && self.state != GameState::DealerTurn
    }

    pub fn hand_in_words(&self) -> String {
        match self.state {
            GameState::PlaceBet => "Bet [amount]".to_string(),

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
                "*You've got Blackjack!*\nDealer: {}\nPlayer: {}\n_Payout_: {}",
                join_cards(&self.dealer_hand),
                join_cards(&self.player_hand),
                self.payout.separated_string()
            ),

            GameState::Push => format!(
                "*It's a Push!*\nDealer: {}\nPlayer: {}\n_Payout_: {}",
                join_cards(&self.dealer_hand),
                join_cards(&self.player_hand),
                self.payout.separated_string()
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
                "*You Won!*\nDealer: {}\nPlayer: {}\n_Payout_: {}",
                join_cards(&self.dealer_hand),
                join_cards(&self.player_hand),
                self.payout.separated_string()
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
        let g = Game::new(&String::from("test"));
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
        test_hand.push(Card::new(Suit::Hearts, "A".to_string(), 11));
        test_hand.push(Card::new(Suit::Spades, "5".to_string(), 5));
        test_hand.push(Card::new(Suit::Spades, "Q".to_string(), 10));
        test_hand.push(Card::new(Suit::Spades, "9".to_string(), 9));

        assert_eq!(score_hand(&test_hand), 25);
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
