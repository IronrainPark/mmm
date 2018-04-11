extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use ex_api;
use ex_api::Api;
use ex_api::ApiCall;
use ex_api::exchanges::BitThumb;
use net_client::Client;

pub struct TradingMachine {
    connection: Client,
    exchange: BitThumb,
    api: ApiCall,
}

impl TradingMachine {
    pub fn new() -> TradingMachine {
        TradingMachine {
            connection: Client::new(),
            exchange: ex_api::exchanges::BitThumb::new("https://api.bithumb.com/public/ticker/".to_string()),
            api: ApiCall::new(),
        }
    }

    pub fn start_trading(&mut self, coin: &str) {
        self.api.current_price(&mut self.connection, &self.exchange, coin);
    }
}