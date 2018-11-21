extern crate dotenv;
extern crate slack;

use dotenv::dotenv;
use slack::RtmClient;

mod bot;

fn main() {
    dotenv().ok();

    let api_key = std::env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN was not found.");

    let mut bot = bot::CryptoJackBot{};
    let r = RtmClient::login_and_run(&api_key, &mut bot);

    println!("{:?}", r);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
} 
}