extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

mod net_client;
mod ex_api;

use ex_api::Api;
use ex_api::ApiClient;
use ex_api::exchanges;


fn main() {
    let api_client = ApiClient::new();    
    let mut net = net_client::Client::new();
    let bitthumb = exchanges::BitThumb::new("https://api.bithumb.com/public/ticker/".to_string());

    for i in 1..10 {
        api_client.current_price(&mut net, &bitthumb, "BTC");
    }

    
}
