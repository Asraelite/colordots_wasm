import * as wasm from './wasm.mjs';
import * as graphics from './graphics.mjs';

let t = 0;

window.addEventListener('load', init);

async function init() {
	await wasm.init();
	graphics.init();

	tick();
}

function tick() {
	graphics.clear();
	wasm.tick();
	//requestAnimationFrame(tick);
}
