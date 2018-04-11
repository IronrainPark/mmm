extern crate sciter;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use sciter::Element;
use self::sciter::dom::event::*;
use self::sciter::dom::HELEMENT;

mod net_client;
mod ex_api;
mod trading;

use ex_api::Api;
use ex_api::ApiCall;
use ex_api::exchanges;

struct FireEvent;

impl sciter::EventHandler for FireEvent {	
	fn on_event(&mut self, _root: HELEMENT, source: HELEMENT, _target: HELEMENT, code: BEHAVIOR_EVENTS, phase: PHASE_MASK, _reason: EventReason) -> bool {
		if phase != PHASE_MASK::BUBBLING {
			return false;
		}
		if code == BEHAVIOR_EVENTS::BUTTON_CLICK {
			let source = Element::from(source);

			if let Some(id) = source.get_attribute("id") {
				if id == "play" {
					println!("play 버튼 클릭");
					call_start_trading();
					return true;

				} else if id == "stop" {
					println!("stop 버튼 클릭");
					return true;
				}
			};
		};
		false
	}
}



fn call_start_trading() {
	let mut tm = trading::TradingMachine::new();
	tm.start_trading("BTC");
}


fn main() {
	let html = include_bytes!("gui/ui.htm");
	let mut frame = sciter::Window::with_size((1016, 638), sciter::window::SCITER_CREATE_WINDOW_FLAGS::SW_MAIN);
	frame.event_handler(FireEvent);
	frame.load_html(html, None);
	frame.run_app();
}