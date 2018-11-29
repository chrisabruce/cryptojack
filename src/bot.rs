use std::collections::HashMap;

use regex::Regex;
use slack::{self, Event, RtmClient};

use games::blackjack;

pub struct CryptoJackBot {
    pub name: String,
    pub active_games: HashMap<String, blackjack::Game>,
    pub completed_games: Vec<blackjack::Game>,
}

impl CryptoJackBot {
    pub fn new(name: &String) -> CryptoJackBot {
        let mut d = blackjack::Deck::new(1);
        d.shuffle();
        print!("You got the {:?}", d);

        CryptoJackBot {
            name: name.clone(),
            active_games: HashMap::new(), //TODO: Persist
            completed_games: Vec::new(),  //TODO: Persist
        }
    }

    fn on_message(&mut self, client: &RtmClient, message: slack::Message) {
        match message {
            slack::Message::Standard(message) => {
                print!("message");
                if let Some(command) = has_command(&message.text) {
                    if let Some(output) = self.eval_command(command, &message.user) {
                        let channel_id = message.channel.unwrap();
                        let _ = client.sender().send_message(&channel_id, &output);
                    } else if let Some(output) = has_bot_mention(&self, &message.text) {
                        let channel_id = message.channel.unwrap();
                        let _ = client.sender().send_message(&channel_id, &output);
                    }
                };
            }
            _ => println!("other"),
        }
    }

    fn eval_command(&mut self, command: String, user: &Option<String>) -> Option<String> {
        match command.to_lowercase().as_str() {
            "bet" => {
                if let Some(u) = user {
                    let g = self.active_games.entry(u.clone()).or_insert(blackjack::Game::new(&String::from("temp"), 500));
                    return Some(g.hand_in_words());
                }
                None
            },
            "hit" => {
                if let Some(u) = user {
                    let mut g = self.active_games.get_mut(u).unwrap();
                    g.hit();
                    return Some(g.hand_in_words());
                }
                None
            }
            _ => None,
        }
    }
}

impl slack::EventHandler for CryptoJackBot {
    fn on_event(&mut self, client: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
        match event {
            Event::Message(reference) => self.on_message(client, *reference),
            _ => println!("other:"),
        }
    }

    fn on_close(&mut self, _client: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, _client: &RtmClient) {
        println!("on_connect");
    }
}

fn has_command(message: &Option<String>) -> Option<String> {
    match message {
        &Some(ref text) => {
            let re = Regex::new(r"/blackjack (?P<command>.*?)$").unwrap();
            match re.captures(&text) {
                Some(capture) => Some(String::from(&capture["command"])),
                _ => None,
            }
        }
        _ => None,
    }
}

fn has_bot_mention(bot: &CryptoJackBot, message: &Option<String>) -> Option<String> {
    match message {
        &Some(ref text) => {
            let re = Regex::new(r"@(?P<bot>[\w_]+)").unwrap();
            for caps in re.captures_iter(&text) {
                if bot.name == &caps["bot"] {
                    return Some(String::from("Hi there!"));
                };
            }
            None
        }
        _ => None,
    }
}
