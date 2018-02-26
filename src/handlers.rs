use binance::api::Binance;
use binance::market::Market;

use reqwest;
use serde_json;

pub trait Handler {
    fn handle(&self, &[String]) -> String;
}

pub struct CryptoHandler {
    market: Market,
}

impl CryptoHandler {
    pub fn new() -> CryptoHandler {
        let market = Binance::new(None, None);
        CryptoHandler { market }
    }

    fn fetch_market_price(&self, sym: &str) -> Result<(f64, String), String> {
        let price: f64;
        let change: String;

        match self.market.get_price(sym) {
            Ok(result) => price = result,
            Err(info) => return Err(format!("Could not get price: {:?}", info)),
        }
        match self.market.get_24h_price_stats(sym) {
            Ok(result) => change = result.price_change_percent,
            Err(info) => return Err(format!("Could not get price stats: {:?}", info)),
        }

        Ok((price, change))
    }
}

impl Handler for CryptoHandler {
    fn handle(&self, syms: &[String]) -> String {
        if syms.len() == 0 {
            return String::from(
                "Find listed coins at https://info.binance.com/en. USD is USDT, i.e. BTCUSDT",
            );
        }

        let mut response = String::new();
        for sym in syms {
            if response.len() > 0 {
                response += "\n";
            }
            let norm = sym.to_uppercase();
            match self.fetch_market_price(norm.as_str()) {
                Ok((price, change)) => {
                    response += &format!("{}: price: {} | 24hr change: {}%", norm, price, change);
                }
                Err(msg) => {
                    response += &format!("{}: Unable to get price info: {}", norm, msg);
                }
            }
        }

        response
    }
}

pub struct StocksHandler;

impl StocksHandler {
    pub fn new() -> StocksHandler {
        StocksHandler {}
    }
}

impl Handler for StocksHandler {
    fn handle(&self, syms: &[String]) -> String {
        let mut response = String::new();
        if syms.len() == 0 {
            return response;
        }

        for sym in syms {
            if response.len() > 0 {
                response += "\n";
            }

            let norm = sym.to_uppercase();
            let url = format!("https://api.iextrading.com/1.0/stock/{}/quote", norm);
            match reqwest::get(url.as_str()) {
                Ok(mut quote_response) => {
                    match quote_response.json::<serde_json::Value>() {
                        Ok(quote) => {
                            let change_percent = match quote["changePercent"].as_f64() {
                                Some(change) => change * 100.0,
                                _ => 0.0,
                            };

                            response += &format!(
                                "{}: price: ${} | 24h change: ${} ({:.2}%)",
                                norm,
                                quote["latestPrice"],
                                quote["change"],
                                change_percent
                            );
                        }
                        Err(err) => response += &format!("{}: error: {:?}", norm, err),
                    };
                }
                Err(err) => response += &format!("{}: error: {:?}", norm, err),
            };
        }

        response
    }
}

pub struct BangHandler;

impl BangHandler {
    pub fn new() -> BangHandler {
        BangHandler {}
    }
}

impl Handler for BangHandler {
    fn handle(&self, _: &[String]) -> String {
        String::from("bang bang")
    }
}
