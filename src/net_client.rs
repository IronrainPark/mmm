extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use hyper::Client as HyperClient;
use hyper::Uri;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use std::io::{self, Write};
use futures::{Future, Stream};
use tokio_core::reactor::Handle;

pub struct Client {
    core: Core,
}

impl Client {
    pub fn new() -> Client {
        Client {
            core: Core::new().unwrap(),
        }
    }

    pub fn build_one(&self) -> HyperClient<HttpsConnector<hyper::client::HttpConnector>> {
        let handle = self.core.handle();
        HyperClient::configure()
                .connector(HttpsConnector::new(4, &handle).unwrap())
                .build(&handle)
    }

    pub fn get(&mut self, uri: &str) {
        let client = self.build_one();
        let req = client.get(uri.parse().unwrap()).and_then(|res| {
            println!("Response: {}", res.status());

            res.body().for_each(|chunk| {
                io::stdout().write_all(&chunk).map_err(From::from)
            })
        });
        self.core.run(req).unwrap();         
    }
}