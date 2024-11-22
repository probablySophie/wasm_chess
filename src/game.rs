use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::prelude::*;

const DELTA_MILLISECONDS: f64 = 75.;

#[wasm_bindgen]
pub struct Game
{
	board: [[char; 8]; 8],
	time: u32,
}

#[wasm_bindgen]
impl Game
{
	pub fn new() -> Game
	{
		let mut board = [
				['♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖'],
				['♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙'],
				[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
				[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
				[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
				[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
				['♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟'],
				['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜'],
			];
		
		// TODO: Check the URL query for moves & update as appropriate
		
		Game {
			board,
			time: 0,
		}
	}
		
	pub fn update(&mut self, deltatime: u32)
	{
		self.time += deltatime;

	}

	pub fn build_chessboard(&mut self, document: &web_sys::Document, chessboard: &web_sys::Element)
	{
		for row in self.board
		{
			let new_row = document.create_element("tr").expect("Failed to create new row element");
			for col in row
			{
				let new_item = document.create_element("td").expect("Failed to create new element");
				new_item.set_text_content(Some(&col.to_string()));
				new_row.append_child(&new_item).expect("Failed to append item to new row");
			}
			chessboard.append_child(&new_row).expect("Failed to append row to chessboard");
		}
	}

	/// Draw the game
	pub fn draw(&mut self)
	{
		let window = web_sys::window().expect("Failed to get window");
		let document = window.document().expect("Failed to get document");

		let chessboard = document.get_element_by_id("chessboard").expect("Failed to get chessboard from document");
		
		if chessboard.child_element_count() == 0
		{
			self.build_chessboard(&document, &chessboard);
		}
	}

	pub fn mouse_moved(&mut self, x: i32, y: i32)
	{
		//
	}
	pub fn mouse_pressed(&mut self, x: i32, y: i32)
	{
		// TODO: Select an element & make it movable
	}
}
