import init, { Game } from "./build/wasm_chess.js"

let lastFrame = performance.timeOrigin + performance.now();

init().then(() => {
	const game = Game.new();
	game.draw();

	canvas.addEventListener("mousemove", (event) => mouse_event(game, event));
	canvas.addEventListener("mousedown", (event) => mouse_event(game, event));
});
function frame(game)
{
	let now = (performance.timeOrigin + performance.now());
	let delta = now - lastFrame;
	lastFrame = now;

	game.update(delta);
	game.draw(context);
	requestAnimationFrame(() => frame(game));
}

function mouse_event(game, event)
{
	let x = event.clientX;
	let y = event.clientY;

	switch (event.type)
	{
		case "mousemove": game.mouse_moved(x, y); break;
		case "mousedown": game.mouse_pressed(x, y); break;
	}
}
