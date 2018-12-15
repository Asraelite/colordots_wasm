let canvas, context, images;

export function init() {
	canvas = document.getElementsByTagName('canvas')[0];
	context = canvas.getContext('2d');

	canvas.width = window.innerWidth;
	canvas.height = window.innerHeight;
}

export function getCanvasSize() {
	return [canvas.width, canvas.height];
}

export function setFill(r, g, b) {
	context.fillStyle = `rgb(${r},${g},${b})`;
}

export function setStroke(r, g, b, width) {
	if (width > 0) context.lineWidth = width;
	context.strokeStyle = `rgb(${r},${g},${b})`;
}

export function fillRect(x, y, w, h) {
	context.fillRect(x, y, w, h);
}

export function fillCircle(x, y, r) {
	context.beginPath();
	context.arc(x, y, r, 0, 2 * Math.PI);
	context.closePath();
	context.fill();
}

export function beginPath() {
	context.beginPath();
}

export function moveTo(x, y) {
	context.moveTo(x, y);
}

export function lineTo(x, y) {
	context.lineTo(x, y);
}

export function stroke() {
	context.stroke();
}

export function clear() {
	context.clearRect(0, 0, canvas.width, canvas.height);
}
