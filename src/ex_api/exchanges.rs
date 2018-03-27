use hyper::Uri;

pub trait Exchange {
    fn current_price_uri(&self, coin: &str) -> String;
}

pub struct BitThumb {
    current_price_uri: String,
}

impl BitThumb {
    pub fn new(current_price_uri: String) -> BitThumb {
        BitThumb {
            current_price_uri: current_price_uri,
        }
    }
}

impl Exchange for BitThumb {
    fn current_price_uri(&self, coin: &str) -> String {
        let mut uri = String::from(self.current_price_uri.clone());
        uri.push_str(coin);
        uri
    }
}
