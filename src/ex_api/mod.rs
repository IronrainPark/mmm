pub mod exchanges;

use net_client;
use ex_api;

pub trait Api {
    fn current_price<E>(&self, net: &mut net_client::Client, api: &E, coin: &str) -> f64
        where E: ex_api::exchanges::Exchange;
}

pub struct ApiCall {}

impl ApiCall {
    pub fn new() -> ApiCall {
        ApiCall {}
    }
}

impl Api for ApiCall {
    fn current_price<E>(&self, net: &mut net_client::Client, api: &E, coin: &str) -> f64 
        where E: ex_api::exchanges::Exchange {
        net.get(&api.current_price_uri(coin));
        0.3
    }
}