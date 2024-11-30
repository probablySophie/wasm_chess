import init, { Game } from "./build/wasm_chess.js"

const board = document.getElementById("chessboard");
let lastFrame = performance.timeOrigin + performance.now();

init().then(() => {
	const game = Game.new();
	game.draw();

	board.addEventListener("mousemove", (event) => mouse_event(game, event));
	board.addEventListener("mousedown", (event) => mouse_event(game, event));
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
	let rect = board.getBoundingClientRect();
	let x = event.clientX - rect.left;
	let y = event.clientY - rect.top;

	switch (event.type)
	{
		case "mousemove": game.mouse_moved(x, y); break;
		case "mousedown": game.mouse_pressed(x, y, board); break;
	}
}
