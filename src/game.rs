use wasm_bindgen::prelude::*;

use crate::{board, engine::js, piece};

const DELTA_MILLISECONDS: f64 = 75.;

#[wasm_bindgen]
#[derive(PartialEq, Eq)]
pub enum State
{
	NewTurn,
	Selected,
	Moving,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player
{
	White,
	Black
}

#[wasm_bindgen]
pub struct Game
{
	board: [[char; 8]; 8],
	time: u32,
	state: State,
	turn: Player,
	selected: Option<(usize, usize)>
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
			state: State::NewTurn,
			turn: Player::White,
			selected: None
		}
	}
		
	pub fn update(&mut self, deltatime: u32)
	{
		self.time += deltatime;

	}

	#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
	pub fn build_chessboard(&mut self, document: &web_sys::Document, chessboard: &web_sys::Element)
	{
		for (row_i, row) in self.board.iter().enumerate()
		{
			let new_row = document.create_element("tr").expect("Failed to create new row element");
			for (col_i, col) in row.iter().enumerate()
			{
				let new_item = document.create_element("td")
					.expect("Failed to create new element");
				new_item.set_text_content(Some(&col.to_string()));
				
				let _ = new_item.set_attribute(
					"location", 
					&board::get_square_name(col_i as i32, row_i as i32).clone()
				);
				// Add the square to the row
				new_row.append_child(&new_item).expect("Failed to append item to new row");
			}
			// Add the row to the board
			chessboard.append_child(&new_row).expect("Failed to append row to chessboard");
		}
	}

	pub fn refresh_board(&self, chessboard: &web_sys::Element)
	{
		let rows = chessboard.children();

		for y in 0..=chessboard.child_element_count()
		{
			let Some(row) = rows.get_with_index(y)
			else { continue };
			let columns = row.children();

			for x in 0..=row.child_element_count()
			{
				let Some(square) = columns.get_with_index(x)
				else { continue };

				square.set_text_content( 
					Some ( &self.board[y as usize][x as usize].to_string() ) 
				);
			}
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

	
	pub fn mouse_pressed(&mut self, x: i32, y: i32, chessboard: &web_sys::Element)
	{
		// console_log!("Clicked on #{} at {x}-{y}", chessboard.id());
		
		let domrect = chessboard.get_bounding_client_rect();
		// Turn the x,y coords into square numbers
		let (rel_x, rel_y) = get_square_pressed(
			domrect.width(), 
			domrect.height(), 
			f64::from(x), 
			f64::from(y)
		);
		// For checking what team the selected piece belongs to
		let selected_char = self.board[rel_y][rel_x];

		// The element we clicked on
		let Some(clicked) = get_clicked(
			chessboard, 
			u32::try_from(rel_x).expect(":("), 
			u32::try_from(rel_y).expect(":("))
		else {
			console_log!("Failed to get clicked at {rel_x}-{rel_y}");
			return
		};
		// If we're selecting & selecting the right colour
		if self.state == State::NewTurn // if we're selecting
			&& piece::get_colour(selected_char) == Some(self.turn) // clicked the right colour
			&& self.selected.is_none() // and haven't already selected anything
		{
			// Update the selected guy's classname to "selected"
			clicked.set_class_name("selected");   // highlight the square
			self.selected = Some((rel_x, rel_y)); // save which we selected
			self.state = State::Selected;         // and change our state
			
		}

		else if self.state == State::Selected
		{
			// Did we click on either an empty space or other player's piece
			if piece::get_colour(selected_char) != Some(self.turn)
			{
				let Some(selected) = self.selected
				else 
				{	// Reset
					console_log!("Nothing selected??"); 
					self.state = State::NewTurn; 
					return
				};
				// TODO: Check if its a valid move
				//     * If not, flash something red maybe?
				self.make_move(selected.0, selected. 1, rel_x, rel_y);
				self.refresh_board(chessboard);

				board::unselect_all(chessboard);
				self.selected = None;
			}
		}
		// console_log!("Mouse pressed {} {}", x, y);
		// console_log!("Square [{},{}] {}", rel_x, rel_y, self.board[rel_y][rel_x]);
	}

	fn make_move(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize)
	{
		self.board[to_y][to_x] = self.board[from_y][from_x];
		self.board[from_y][from_x] = ' ';
		
		// Swap turns
		match self.turn {
			Player::White => self.turn = Player::Black,
			Player::Black => self.turn = Player::White,
		}
		self.state = State::NewTurn;
	}
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn get_square_pressed(board_width: f64, board_height: f64, x: f64, y: f64) -> (usize, usize)
{
	(
		( ( x / (board_width  / 8.) ) as u16 ).into(),
		( ( y / (board_height / 8.) ) as u16 ).into()
	)
}

fn get_clicked(chessboard: &web_sys::Element, rel_x: u32, rel_y: u32) -> Option<web_sys::Element>
{
	let row = chessboard.children().get_with_index(rel_y)?;
	row.children().get_with_index(rel_x)	
}

