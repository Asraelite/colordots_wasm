#![feature(nll)]
#![feature(label_break_value)]

use std::cell::RefCell;
use std::panic;

use backtrace;

mod graphics;
mod js;
mod world;

use crate::world::{FloatSize, World};

thread_local! {
	static STATE: RefCell<GlobalState> = RefCell::new(GlobalState::new());
}

pub struct GlobalState {
	world: World,
}

impl GlobalState {
	pub fn new() -> Self {
		Self {
			world: World::new(),
		}
	}
}

#[no_mangle]
extern "C" fn init(width: FloatSize, height: FloatSize) {
	set_panic_hook();

	STATE.with(|state_local_key| {
		let mut state = state_local_key.borrow_mut();
		state.world.init(width, height);
	});
}

#[no_mangle]
extern "C" fn tick() {
	STATE.with(|state_local_key| {
		let mut state = state_local_key.borrow_mut();
		state.world.tick();
		graphics::render(&state);
	});
}

fn set_panic_hook() {
	panic::set_hook(Box::new(|panic_info| {
		let backtrace = backtrace::Backtrace::new();
		let payload = panic_info.payload();

		let message = if let Some(message) = payload.downcast_ref::<String>() {
			message.as_str()
		} else if let Some(message) = payload.downcast_ref::<&str>() {
			message
		} else {
			"Unknown panic payload type"
		};

		let location_string = if let Some(location) = panic_info.location() {
			format!("{} {}", location.file(), location.line())
		} else {
			String::from("Unknown location")
		};

		js::console_log(format!(
			"Panic: {:?}\n\tat {}\n{:?}",
			message, location_string, backtrace
		));
	}));
}
