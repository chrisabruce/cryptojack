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

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck { cards: Vec::new() };

        let suits = vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];

        for suit in suits {
            for i in 2..10 {
                deck.cards.push(Card {
                    suit: suit,
                    name: i.to_string(),
                    value: i,
                    alt_value: i,
                });
            }
            deck.cards.push(Card {
                suit: suit,
                name: String::from("Ace"),
                value: 11,
                alt_value: 1,
            });
            deck.cards.push(Card {
                suit: suit,
                name: String::from("King"),
                value: 10,
                alt_value: 10,
            });
            deck.cards.push(Card {
                suit: suit,
                name: String::from("Queen"),
                value: 10,
                alt_value: 10,
            });
            deck.cards.push(Card {
                suit: suit,
                name: String::from("Jack"),
                value: 10,
                alt_value: 10,
            });
        }
        deck
    }
}
