#![allow(unused)]

use super::GlobalState;
use crate::js;

struct Color {
	red: u8,
	green: u8,
	blue: u8,
}

impl Color {
	pub fn new(red: u8, green: u8, blue: u8) -> Self {
		Self { red, green, blue }
	}
}

pub fn render(state: &GlobalState) {
	let (canvas_width, canvas_height) = state.world.size.as_tuple();
	js::canvas_set_fill_rgb(0, 0, 0);
	js::canvas_fill_rect(0.0, 0.0, canvas_width as f64, canvas_height as f64);

	for particle in state.world.particles.iter() {
		let (pos_x, pos_y) = particle.position.as_tuple();
		let render_size = 1.0;
		//let color = Color::new(255, 255, 255);
		let hue = ((particle.color % 1.0) * 360.0).floor();

		// Wrapping in cast to u8 expected
		js::canvas_set_fill_hsl(hue as u16, 100, 75);
		js::canvas_fill_rect(
			(pos_x - render_size / 2.0) as f64,
			(pos_y - render_size / 2.0) as f64,
			render_size as f64,
			render_size as f64,
		);
	}
}
