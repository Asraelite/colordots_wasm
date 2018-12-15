import * as graphics from './graphics.mjs';

let mod;
let rustExports = {};

const WASM_FILE = 'main.wasm';

export async function init(...args) {
	let imports = {
		env: rustExports
	};

	mod = (await WebAssembly.instantiateStreaming(fetch(WASM_FILE),
		imports)).instance.exports;

	window.mod = mod;

	try {
		mod.init(...args);
	} catch(_) {
		wasmError();
		return false;
	}

	return mod;
};

export function tick() {
	try {
		mod.tick();
		return true;
	} catch(_) {
		wasmError();
		return false;
	}
}

function wasmError() {
	console.error('Error in WASM');
}

// Functions callable from Rust.

rustExports.js_log = (pointer, length) => {
	let str = readString(pointer, length);
	console.log('%c' + str, 'font-weight: 700; color: #dea584;');
};

rustExports.js_set_fill = graphics.setFill;
rustExports.js_set_stroke = graphics.setStroke;
rustExports.js_fill_rect = graphics.fillRect;
rustExports.js_fill_circle = graphics.fillCircle;
rustExports.js_begin_path = graphics.beginPath;
rustExports.js_move_to = graphics.moveTo;
rustExports.js_line_to = graphics.lineTo;
rustExports.js_stroke = graphics.stroke;

rustExports.sin = Math.sin;
rustExports.cos = Math.cos;
rustExports.sqrt = Math.sqrt;
rustExports.atan2 = Math.atan2;
rustExports.js_random = Math.random;
rustExports.fmod = (num, div) => num % div;

// Helper functions

function readString(pointer, length) {
	let memory = new Uint8Array(mod.memory.buffer, pointer, length);
	return (new TextDecoder("UTF-8")).decode(memory);
}
