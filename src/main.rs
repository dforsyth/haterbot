// extern crate binance;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde_json;
extern crate simplelog;
extern crate slack;

use simplelog::{Config, LevelFilter, TermLogger};
use std::env;

mod bot;
mod handlers;

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    let name = env::var("HBOT_NAME").expect("No username");
    let token = env::var("HBOT_SLACK_API_TOKEN").expect("No slack api token");
    let icon = env::var("HBOT_ICON").expect("No icon");

    let config = bot::HaterBotConfig { name, icon, token };

    let mut haterbot = bot::HaterBot::new(config);
    haterbot.add_command("crypto", Box::new(handlers::CryptoHandler::new()));
    haterbot.add_command("stocks", Box::new(handlers::StocksHandler::new()));
    haterbot.add_command("bang", Box::new(handlers::BangHandler::new()));

    info!("Running bot");
    haterbot.run();
}
