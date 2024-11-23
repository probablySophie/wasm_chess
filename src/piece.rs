use std::ops::BitXor;

use crate::game::Player;

// TODO: Testing for move validation
//     * Can't test EVERY move case, but can at least test a valid and invalid test

#[path = "test/piece_test.rs"] mod test;

fn validate_rook(board: [[char; 8]; 8], piece_x: usize, piece_y: usize, move_x: usize, move_y: usize) -> bool
{
	// We can only move in X or Y so if we moved in both then return false
	// This also catches neither, but that's not a move so would be invalid anyway!
	if ! (piece_x.abs_diff(move_x) != 0).bitxor( piece_y.abs_diff(move_y) != 0 )
	{
		return false
	}
	// TODO: Check if the move is unblocked
	false
}
fn validate_bishop(board: [[char; 8]; 8], piece_x: usize, piece_y: usize, move_x: usize, move_y: usize) -> bool
{
	// We have to have moved the same amount on the X & Y axis
	if piece_x.abs_diff(move_x) != piece_y.abs_diff(move_y)
	{
		return false
	}
	// TODO: Check if the move is unblocked
	false
}

pub fn validate_move(board: [[char; 8]; 8], piece_x: usize, piece_y: usize, move_x: usize, move_y: usize) -> bool
{
	// We've already confirmed that we're not actively moving ONTO a friendly piece
	// So now we just need to make sure we're moving in a legit way & not through a friend
	let piece = board[piece_y][piece_x];
	
	match piece
	{
		'♖' | '♜' => // rook
		{
			// Unblocked lefty righty upy downy
			validate_rook(board, piece_x, piece_y, move_x, move_y)
		},
		'♞' | '♘' => // knight
		{
			// Make sure we're moving 2 on one axis, and one on the other
			(piece_x.abs_diff(move_x) == 2 && piece_y.abs_diff(move_y) == 1)
			||
			(piece_x.abs_diff(move_x) == 1 && piece_y.abs_diff(move_y) == 2)
		},
		'♗' | '♝' => // bishhop
		{
			// Unblocked X
			validate_bishop(board, piece_x, piece_y, move_x, move_y)
		},
		'♕' | '♛' => // queen
		{
			// Unblocked X+
			// rook + bishop
			validate_rook(board, piece_x, piece_y, move_x, move_y) 
			|| // or
			validate_bishop(board, piece_x, piece_y, move_x, move_y)
		},
		'♔' | '♚' => // king
		{
			piece_x.abs_diff(move_x) <= 1// if our change in X is 0 or 1
			&& // and
			piece_y.abs_diff(move_y) <= 1// our change in Y is 0 or 1
			// Then we've moved one space (and allowed diagonals)
		},
		'♙' | '♟' => // pawn
		{
			// If black is going up or white is going down
			if ( piece == '♙' && ( piece_y < move_y ) )
			|| ( piece == '♟' && ( piece_y > move_y ) )
			{
				return false // that's bad
			}
			// If we're only moving 1 & the space is free
			if ( piece_y.abs_diff(move_y) == 1 )
			&& ( board[move_y][move_x] == ' ' )
			{
				return true // Then we're happy
			}
			// TODO: If we're moving 2, make sure its unblocked

			// TODO: En passant!?!
			todo!()
		},
		_ => { false } // uhhhh what are you trying to move???
	}
}

pub fn get_colour(piece: char) -> Option<Player>
{
	let piece_num = u32::from(piece);
	if (9812..=9817).contains(&piece_num)
	{
		return Some(Player::Black)
	}
	// Else
	if (9818..=9823).contains(&piece_num)
	{
		return Some(Player::White)
	}
	// Else
	None
}
