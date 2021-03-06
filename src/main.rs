use dotenv::dotenv;
use helium;
use slack::RtmClient;

pub mod accounts;
pub mod bot;
pub mod db;
pub mod games;

fn main() {
    dotenv().ok();

    let api_key = std::env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN was not found.");
    let bot_name = std::env::var("SLACK_BOT_NAME").expect("SLACK_BOT_NAME was not found.");

    let mut bot = bot::CryptoJackBot::new(&bot_name);
    let r = RtmClient::login_and_run(&api_key, &mut bot);

    println!("{:?}", r);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}
