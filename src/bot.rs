use regex::Regex;
use slack::{self, Event, RtmClient};

use games::blackjack;

pub struct CryptoJackBot {
    pub name: String,
    pub deck: blackjack::Deck,
}

impl CryptoJackBot {
    pub fn new(name: &String) -> CryptoJackBot {
        let mut d = blackjack::Deck::new();
        d.shuffle();
        print!("You got the {:?}", d);

        CryptoJackBot {
            name: name.clone(),
            deck: d,
        }
    }

    fn on_message(&mut self, client: &RtmClient, message: slack::Message) {
        match message {
            slack::Message::Standard(message) => {
                print!("message");
                if let Some(command) = has_command(&message.text) {
                    if let Some(output) = self.eval_command(command) {
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

    fn eval_command(&mut self, command: String) -> Option<String> {
        match command.to_lowercase().as_str() {
            "deal" => Some(self.deck.deal_card().unwrap().to_string()),
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

    fn on_close(&mut self, client: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, client: &RtmClient) {
        println!("on_connect");
    }
}

fn has_command(message: &Option<String>) -> Option<String> {
    match message {
        &Some(ref text) => {
            let re = Regex::new(r"/help (?P<command>.*?)$").unwrap();
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
