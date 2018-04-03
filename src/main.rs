/*
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
*/

extern crate sciter;

use sciter::Element;
use self::sciter::dom::event::*;
use self::sciter::dom::HELEMENT;
use self::sciter::value::Value;

struct FireEvent;

impl sciter::EventHandler for FireEvent {

	fn on_event(&mut self, root: HELEMENT, source: HELEMENT, target: HELEMENT, code: BEHAVIOR_EVENTS, phase: PHASE_MASK, reason: EventReason) -> bool {
		if phase != PHASE_MASK::BUBBLING {
			return false;
		}

		if code == BEHAVIOR_EVENTS::BUTTON_CLICK {

			// `root` points to attached element, usually it is an `<html>`.

			let root = Element::from(root).root();

			let message = root.find_first("#message").unwrap().expect("div#message not found");
			let source = Element::from(source);

			println!("our root is {:?}, message is {:?} and source is {:?}", root, message, source);

			if let Some(id) = source.get_attribute("id") {
				if id == "send" {

					// just send a simple event
					source.send_event(BEHAVIOR_EVENTS::CHANGE, None, Some(message.as_ptr())).expect("Failed to send event");
					return true;

				} else if id == "fire" {

					// fire event with specified params
					let data = Value::from("Rusty param");

					source.fire_event(BEHAVIOR_EVENTS::CHANGE, None, Some(message.as_ptr()), false, Some(data)).expect("Failed to fire event");
					return true;
				}
			};
		};

		false
	}
}

fn main() {
		let html = include_bytes!("fire_event.htm");
		let mut frame = sciter::Window::new();
		frame.event_handler(FireEvent);
		frame.load_html(html, None);
		frame.run_app();
}