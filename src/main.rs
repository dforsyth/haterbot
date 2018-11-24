// extern crate binance;
#[macro_use]
extern crate log;
extern crate reqwest;
#[macro_use]
extern crate serde_json;
extern crate rand;
extern crate simplelog;
extern crate slack;

use simplelog::{Config, LevelFilter, TermLogger};
use std::env;

mod bot;
mod handlers;

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    let token = env::var("HBOT_SLACK_API_TOKEN").expect("No slack api token");
    let name = env::var("HBOT_NAME").unwrap_or("haterbot".to_string());
    let icon = env::var("HBOT_ICON")
        .unwrap_or("https://i.imgur.com/d4ddm4a.jpg".to_string());

    let config = bot::HaterBotConfig { name, icon, token };

    let mut haterbot = bot::HaterBot::new(config);
    haterbot.add_command("crypto", Box::new(handlers::CryptoHandler::new()));
    haterbot.add_command("stocks", Box::new(handlers::StocksHandler::new()));
    haterbot.add_command("bang", Box::new(handlers::BangHandler::new()));
    haterbot.add_command(
        "timon",
        Box::new(handlers::RandomHandler::new(
            vec!["Yes officer, that post right there.".to_string()],
            vec!["https://i.imgur.com/iOPRp0B.jpg".to_string()],
        )),
    );

    info!("Running bot");
    haterbot.run();
}
