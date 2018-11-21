extern crate dotenv;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let api_key = std::env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN was not found.");

    print!("Hello from CryptoJack! with {:?}", api_key)   
}