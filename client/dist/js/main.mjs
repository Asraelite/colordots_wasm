import * as wasm from './wasm.mjs';
import * as graphics from './graphics.mjs';

let t = 0;

window.addEventListener('load', init);

async function init() {
	graphics.init();

	let success = await wasm.init(...graphics.getCanvasSize());
	if (!success) return;

	tick();
}

function tick() {
	graphics.clear();
	if (!wasm.tick()) return;
	requestAnimationFrame(tick);
}
