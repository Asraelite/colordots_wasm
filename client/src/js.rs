#![allow(unused)]

extern "C" {
	fn js_log(str_ptr: *const u8, length: u32);
	fn js_set_fill(r: u8, g: u8, b: u8);
	fn js_set_stroke(r: u8, g: u8, b: u8, width: f64);
	fn js_fill_rect(x: f64, y: f64, w: f64, h: f64);
	fn js_fill_circle(x: f64, y: f64, r: f64);
	fn js_begin_path();
	fn js_move_to(x: f64, y: f64);
	fn js_line_to(x: f64, y: f64);
	fn js_stroke();
}

use std::borrow::Borrow;

pub fn console_log<T: Borrow<str>>(message: T) {
	let slice = message.borrow().as_bytes();
	unsafe {
		js_log(slice.as_ptr(), slice.len() as u32);
	}
}

pub fn canvas_set_fill(r: u8, g: u8, b: u8) {
	unsafe { js_set_fill(r, g, b) }
}

pub fn canvas_set_stroke(r: u8, g: u8, b: u8, width: f64) {
	unsafe { js_set_stroke(r, g, b, width) }
}

pub fn canvas_fill_rect(x: f64, y: f64, w: f64, h: f64) {
	unsafe { js_fill_rect(x, y, w, h) }
}

pub fn canvas_fill_circle(x: f64, y: f64, r: f64) {
	unsafe { js_fill_circle(x, y, r) }
}

pub fn canvas_begin_path() {
	unsafe { js_begin_path() }
}

pub fn canvas_move_to(x: f64, y: f64) {
	unsafe { js_move_to(x, y) }
}

pub fn canvas_line_to(x: f64, y: f64) {
	unsafe { js_line_to(x, y) }
}

pub fn canvas_stroke() {
	unsafe { js_stroke() }
}
