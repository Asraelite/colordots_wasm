#![allow(unused)]

extern "C" {
	fn js_log(str_ptr: *const u8, length: u32);
	fn js_set_fill(color_format: u8, r: u16, g: u8, b: u8);
	fn js_set_stroke(color_format: u8, r: u8, g: u8, b: u8, width: f64);
	fn js_fill_rect(x: f64, y: f64, w: f64, h: f64);
	fn js_fill_circle(x: f64, y: f64, r: f64);
	fn js_begin_path();
	fn js_move_to(x: f64, y: f64);
	fn js_line_to(x: f64, y: f64);
	fn js_stroke();
	fn js_random() -> f64;
}

use std::borrow::Borrow;

pub fn console_log<T: Borrow<str>>(message: T) {
	let slice = message.borrow().as_bytes();
	unsafe {
		js_log(slice.as_ptr(), slice.len() as u32);
	}
}

pub fn canvas_set_fill_rgb(r: u8, g: u8, b: u8) {
	unsafe { js_set_fill(0, r as u16, g, b) }
}

pub fn canvas_set_fill_hsl(h: u16, s: u8, l: u8) {
	unsafe { js_set_fill(1, h, s, l) }
}

pub fn canvas_set_stroke(r: u8, g: u8, b: u8, width: f64) {
	unsafe { js_set_stroke(0, r, g, b, width) }
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

pub fn math_random() -> f64 {
	unsafe { js_random() }
}
