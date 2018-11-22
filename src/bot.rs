use regex::Regex;
use slack::{self, Event, RtmClient};

pub struct CryptoJackBot {
    pub name: String,
}

impl CryptoJackBot {
    pub fn new(name: &String) -> CryptoJackBot {
        CryptoJackBot {
            name: name.clone(),
        }
    }

    fn on_message(&self, client: &RtmClient, message: slack::Message) {
        match message {
            slack::Message::Standard(message) => {
                print!("message");
                if let Some(output) = has_bot_mention(&self, &message.text) {
                    let channel_id = message.channel.unwrap();
                    let _ = client.sender().send_message(&channel_id, &output);
                };
            }
            _ => println!("other"),
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
