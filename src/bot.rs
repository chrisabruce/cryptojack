
use slack::{self, Event, RtmClient};

pub struct CryptoJackBot;


impl slack::EventHandler for CryptoJackBot {
    fn on_event(&mut self, client: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
    }

    fn on_close(&mut self, client: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, client: &RtmClient) {
        println!("on_connect");
}
}