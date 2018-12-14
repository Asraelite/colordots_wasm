use std::cell::RefCell;

mod js;
mod graphics;

thread_local! {
	static STATE: RefCell<GlobalState> = RefCell::new(GlobalState::new());
}

pub struct GlobalState {
	num: i32,
}

impl GlobalState {
	pub fn new() -> Self {
		Self { num: 0 }
	}
}

#[no_mangle]
extern "C" fn tick() {
	STATE.with(|state_local_key| {
		let mut state = state_local_key.borrow_mut();
		js::console_log(format!("{}", state.num));
		state.num += 1;
	});
}
