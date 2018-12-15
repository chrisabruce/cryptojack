use std::collections::HashMap;

use regex::Regex;
use slack::{self, Event, RtmClient};

use crate::games::blackjack;

pub struct CryptoJackBot {
    pub name: String,
    pub active_games: HashMap<String, blackjack::Game>,
    pub completed_games: Vec<blackjack::Game>,
}

impl CryptoJackBot {
    pub fn new(name: &str) -> CryptoJackBot {
        let mut d = blackjack::Deck::new(1);
        d.shuffle();

        CryptoJackBot {
            name: name.to_string(),
            active_games: HashMap::new(), //TODO: Persist
            completed_games: Vec::new(),  //TODO: Persist
        }
    }

    fn on_message(&mut self, client: &RtmClient, message: slack::Message) {
        match message {
            slack::Message::Standard(message) => {
                print!("message");
                if let Some(command) = has_command(&message.text, &message.channel) {
                    if let Some(output) = self.eval_command(&command, &message.user) {
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

    fn eval_command(&mut self, command: &str, user: &Option<String>) -> Option<String> {
        let command = command.to_lowercase();
        let args: Vec<&str> = command.split(" ").collect();
        if args.len() > 0 {
            if let Some(u) = user {
                let g = self
                    .active_games
                    .entry(u.clone())
                    .or_insert_with(|| blackjack::Game::new(&u));

                let response = match args[0] {
                    "play" => Some(g.hand_in_words()),
                    "bet" if args.len() > 1 => match args[1].parse::<u64>() {
                        Ok(n) => Some(g.bet(n)),
                        Err(_) => Some("Not a valid bet!".to_string()),
                    },
                    "hit" => Some(g.hit()),
                    "stay" => Some(g.stay()),
                    _ => None,
                };

                let is_over = g.is_over();
                let g = g.clone();

                if is_over {
                    self.active_games.remove(u);
                    self.completed_games.push(g.clone());
                };

                return response;
            }
        }
        None
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

fn has_command(message: &Option<String>, channel: &Option<String>) -> Option<String> {
    if let Some(c) = channel {
        let match_str = if c.starts_with('D') {
            r"(?P<command>.*?)$"
        } else {
            r"/blackjack (?P<command>.*?)$"
        };
        return match message {
            Some(text) => {
                let re = Regex::new(match_str).unwrap();
                match re.captures(&text) {
                    Some(capture) => Some(String::from(&capture["command"])),
                    _ => None,
                }
            }
            _ => None,
        };
    }
    None
}

fn has_bot_mention(bot: &CryptoJackBot, message: &Option<String>) -> Option<String> {
    match message {
        Some(text) => {
            let re = Regex::new(r"@(?P<bot>[\w_]+)").unwrap();
            for caps in re.captures_iter(&text) {
                if bot.name == caps["bot"] {
                    return Some(String::from("Hi there!"));
                };
            }
            None
        }
        _ => None,
    }
}
